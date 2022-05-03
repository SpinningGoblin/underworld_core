use rand::Rng;
use std::ops::RangeInclusive;

use crate::components::{
    inventory::Inventory,
    items::{character_item::CharacterItem, item_type::ItemType, location_tag::LocationTag},
};

use super::{
    generator::Generator,
    items::item_generator,
    utils::item_types::{
        type_cannot_be_used_with, type_inherently_multiple, type_is_for_weapon,
        type_is_for_wearable,
    },
};

pub struct InventoryPrototype {
    pub item_types: Vec<ItemType>,
    pub num_equipped_weapons: RangeInclusive<usize>,
    pub num_equipped_wearables: RangeInclusive<usize>,
    pub num_carried_weapons: RangeInclusive<usize>,
    pub num_carried_wearables: RangeInclusive<usize>,
    pub hidden_weapon_chance: usize,
    pub hidden_wearable_chance: usize,
}

impl InventoryPrototype {
    fn equipped_weapons(&self) -> Vec<CharacterItem> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_weapons.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut equipped_weapons: Vec<CharacterItem> = Vec::new();
        let weapon_types: Vec<&ItemType> = self
            .item_types
            .iter()
            .filter(|item_type| type_is_for_weapon(item_type))
            .collect();
        let mut location_tags = LocationTag::weapon_tags();
        for _ in 1..=count {
            if location_tags.is_empty() {
                break;
            }

            let tag_index = rng.gen_range(0..location_tags.len());
            let tag = location_tags.remove(tag_index);
            let index = rng.gen_range(0..weapon_types.len());
            let weapon_type = match &weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = item_generator(weapon_type, true);
            let weapon = generator.generate();

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = type_inherently_multiple(weapon_type);

            equipped_weapons.push(CharacterItem {
                is_multiple: multiple,
                item: weapon,
                is_hidden: hidden_roll <= self.hidden_weapon_chance,
                at_the_ready: tag.eq(&LocationTag::Hand),
                equipped_location: tag,
            })
        }

        equipped_weapons
    }

    fn equipped_wearables(&self) -> Vec<CharacterItem> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_wearables.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut equipped_wearables: Vec<CharacterItem> = Vec::new();
        let mut used_types: Vec<ItemType> = Vec::new();
        let mut wearable_tags = LocationTag::wearable_tags();

        for _ in 1..=count {
            if wearable_tags.is_empty() {
                break;
            }

            let tag_index = rng.gen_range(0..wearable_tags.len());
            let tag = wearable_tags.remove(tag_index);

            let possible_types: Vec<ItemType> = self
                .item_types
                .iter()
                .filter(|item_type| type_is_for_wearable(item_type))
                .filter(|item_type| item_type_is_for_tags(item_type, &tag))
                .filter(|w_t| {
                    // Return true only if it can be used with all of the used_types
                    if used_types.is_empty() {
                        true
                    } else {
                        used_types.iter().all(|w| !type_cannot_be_used_with(w, w_t))
                    }
                })
                .cloned()
                .collect();

            if possible_types.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_types.len());
            let wearable_type = match &possible_types.get(index) {
                Some(it) => *it,
                _ => continue,
            };

            used_types.push(wearable_type.clone());
            let generator = item_generator(wearable_type, true);
            let wearable = generator.generate();
            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = type_inherently_multiple(wearable_type);

            equipped_wearables.push(CharacterItem {
                is_multiple: multiple,
                item: wearable,
                is_hidden: hidden_roll <= self.hidden_wearable_chance,
                at_the_ready: true,
                equipped_location: tag,
            })
        }

        equipped_wearables
    }
}

impl Generator<Inventory> for InventoryPrototype {
    fn generate(&self) -> Inventory {
        let equipped_weapons = self.equipped_weapons();
        let equipped_wearables = self.equipped_wearables();

        Inventory {
            equipment: equipped_weapons
                .into_iter()
                .chain(equipped_wearables.into_iter())
                .collect(),
        }
    }
}

fn item_type_is_for_tags(item_type: &ItemType, tag: &LocationTag) -> bool {
    match *item_type {
        ItemType::Breastplate | ItemType::Shirt | ItemType::Vest => tag.eq(&LocationTag::Body),
        ItemType::Boots | ItemType::PlateBoots => tag.eq(&LocationTag::Feet),
        ItemType::Buckler => tag.eq(&LocationTag::Hand),
        ItemType::Cloak => tag.eq(&LocationTag::Shoulder),
        ItemType::Club
        | ItemType::Hammer
        | ItemType::Mace
        | ItemType::Morningstar
        | ItemType::Whip => tag.eq(&LocationTag::Hand) || tag.eq(&LocationTag::Hip),
        ItemType::Dagger | ItemType::ShortSword | ItemType::Dirk => {
            tag.eq(&LocationTag::Hand)
                || vec![LocationTag::Hip, LocationTag::HipSheath].contains(tag)
        }
        ItemType::Crown | ItemType::PlateHelmet | ItemType::Helm => tag.eq(&LocationTag::Head),
        ItemType::Gloves | ItemType::PlateGauntlets => tag.eq(&LocationTag::Hand),
        ItemType::GreatSword
        | ItemType::Halberd
        | ItemType::Pike
        | ItemType::Shield
        | ItemType::Spear => tag.eq(&LocationTag::Hand) || tag.eq(&LocationTag::Back),
        ItemType::LoinCloth => tag.eq(&LocationTag::Waist),
        ItemType::LongSword => {
            tag.eq(&LocationTag::Hand)
                || vec![LocationTag::Hip, LocationTag::HipSheath].contains(tag)
                || tag.eq(&LocationTag::Back)
        }
        ItemType::Mask => tag.eq(&LocationTag::Face),
        ItemType::Shackles => tag.eq(&LocationTag::Wrist) | tag.eq(&LocationTag::Ankle),
        ItemType::Trousers => tag.eq(&LocationTag::Leg),
    }
}
