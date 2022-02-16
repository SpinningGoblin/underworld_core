use std::ops::RangeInclusive;

use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::components::{
    equipment::{location_descriptor::LocationDescriptor, location_tag::LocationTag},
    inventory::Inventory,
    items::{character_item::CharacterItem, item_type::ItemType},
    tag::Tagged,
};

use super::{generator::Generator, items::item_generator};

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

        let mut used_descriptors: Vec<LocationDescriptor> = Vec::new();
        let mut equipped_weapons: Vec<CharacterItem> = Vec::new();
        let weapon_types: Vec<&ItemType> = self
            .item_types
            .iter()
            .filter(|item_type| item_type.is_for_weapon())
            .collect();
        for _ in 1..=count {
            let index = rng.gen_range(0..weapon_types.len());
            let weapon_type = match &weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = item_generator(weapon_type);
            let weapon = generator.generate();

            let possibilities: Vec<LocationDescriptor> = LocationDescriptor::into_enum_iter()
                .filter(|descriptor| {
                    descriptor.matches_any_location_tags(character_location_tags(weapon_type))
                        && descriptor.matches_any_item_tags(weapon_type.tags())
                })
                .filter(|descriptor| {
                    if used_descriptors.is_empty() {
                        true
                    } else {
                        used_descriptors
                            .iter()
                            .all(|l| !l.unable_to_be_used_with(descriptor))
                    }
                })
                .collect();

            let range = 0..possibilities.len();

            // If we've got nowhere to put the weapon, we can't equip it.
            if range.is_empty() {
                continue;
            }

            let location_index = rng.gen_range(range);
            let equipped_location = possibilities
                .get(location_index)
                .cloned()
                .unwrap_or_default();

            used_descriptors.push(equipped_location.clone());

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = weapon_type.is_multiple();

            equipped_weapons.push(CharacterItem {
                is_multiple: multiple,
                item: weapon,
                is_hidden: hidden_roll <= self.hidden_weapon_chance,
                equipped_location_tags: equipped_location.tags(),
                location_descriptor: equipped_location,
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

        let mut used_descriptors: Vec<LocationDescriptor> = Vec::new();
        let mut equipped_wearables: Vec<CharacterItem> = Vec::new();
        let mut used_types: Vec<ItemType> = Vec::new();
        for _ in 1..=count {
            let possible_types: Vec<ItemType> = self
                .item_types
                .iter()
                .filter(|item_type| item_type.is_for_wearable())
                .filter(|w_t| {
                    // Return true only if it can be used with all of the used_types
                    if used_types.is_empty() {
                        true
                    } else {
                        used_types.iter().all(|w| !w.unable_to_be_used_with(w_t))
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
            let generator = item_generator(wearable_type);
            let wearable = generator.generate();

            let possibilities: Vec<LocationDescriptor> = LocationDescriptor::into_enum_iter()
                .filter(|descriptor| {
                    descriptor.matches_any_location_tags(character_location_tags(wearable_type))
                        && descriptor.matches_any_item_tags(wearable_type.tags())
                })
                .filter(|descriptor| {
                    if used_descriptors.is_empty() {
                        true
                    } else {
                        used_descriptors
                            .iter()
                            .all(|l| !l.unable_to_be_used_with(descriptor))
                    }
                })
                .collect();

            let range = 0..possibilities.len();

            let equipped_location = if range.is_empty() {
                LocationDescriptor::None
            } else {
                let location_index = rng.gen_range(range);
                possibilities
                    .get(location_index)
                    .cloned()
                    .unwrap_or_default()
            };

            if equipped_location != LocationDescriptor::None {
                used_descriptors.push(equipped_location.clone());
            };

            let equipped_location_tags = if equipped_location == LocationDescriptor::None {
                vec![LocationTag::Equipped]
            } else {
                equipped_location.tags()
            };

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = wearable_type.is_multiple();

            equipped_wearables.push(CharacterItem {
                is_multiple: multiple,
                item: wearable,
                is_hidden: hidden_roll <= self.hidden_wearable_chance,
                location_descriptor: equipped_location,
                equipped_location_tags,
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
            items: equipped_weapons
                .into_iter()
                .chain(equipped_wearables.into_iter())
                .collect(),
        }
    }
}

fn character_location_tags(item_type: &ItemType) -> Vec<LocationTag> {
    match *item_type {
        ItemType::Buckler => vec![LocationTag::Hand],
        ItemType::Club => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Dagger => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
        ItemType::Dirk => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
        ItemType::GreatSword => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::Hammer => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::LongSword => vec![
            LocationTag::Hand,
            LocationTag::Hip,
            LocationTag::HipSheath,
            LocationTag::Back,
        ],
        ItemType::Mace => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Morningstar => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Shield => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::ShortSword => {
            vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath]
        }
        ItemType::Whip => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Breastplate => vec![LocationTag::Body],
        ItemType::Mask => vec![LocationTag::Head],
        ItemType::Cloak => vec![LocationTag::Shoulder],
        ItemType::Shirt => vec![LocationTag::Body],
        ItemType::Trousers => vec![LocationTag::Leg],
        ItemType::Crown => vec![LocationTag::Head],
        ItemType::Boots => vec![LocationTag::Feet],
        ItemType::Gloves => vec![LocationTag::Hand],
        ItemType::LoinCloth => vec![LocationTag::Waist],
        ItemType::PlateBoots => vec![LocationTag::Feet],
        ItemType::PlateGauntlets => vec![LocationTag::Hand],
        ItemType::PlateHelmet => vec![LocationTag::Head],
        ItemType::Shackles => vec![LocationTag::Ankle, LocationTag::Wrist],
        ItemType::Vest => vec![LocationTag::Body],
        ItemType::Helm => vec![LocationTag::Head],
    }
}
