use rand::{prelude::ThreadRng, Rng};
use std::ops::RangeInclusive;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    components::{
        damage::{Attack, Defense},
        inventory::Inventory,
        items::{
            CharacterItem, Consumable, Item, ItemType, LocationTag,
            {ConsumableEffect, ConsumableEffectName, LearnSpellEffect},
        },
        spells::SpellName,
        tag::Tag,
    },
    utils::rolls::roll_d100,
};

use super::{
    generator::Generator,
    items::{item_generator, item_generator_for_level},
    utils::item_types::{type_inherently_multiple, type_is_for_weapon, type_is_for_wearable},
};

const GENERATE_SCROLL_CHANCE: i32 = 25;
const WEAPON_NOT_IN_HAND_CHANCE: i32 = 10;

pub struct InventoryPrototype {
    pub item_types: Vec<ItemType>,
    pub num_equipped_weapons: RangeInclusive<usize>,
    pub num_equipped_wearables: RangeInclusive<usize>,
    pub hidden_weapon_chance: i32,
    pub hidden_wearable_chance: i32,
    pub danger_level: u32,
}

impl InventoryPrototype {
    fn equipped_weapons(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
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
        let mut location_tags = LocationTag::not_at_ready_weapon_tags();
        for _ in 1..=count {
            if location_tags.is_empty() {
                break;
            }

            let roll = roll_d100(rng, 1, 0);
            let tag = if roll > WEAPON_NOT_IN_HAND_CHANCE {
                LocationTag::Hand
            } else {
                let tag_index = rng.gen_range(0..location_tags.len());
                location_tags.remove(tag_index)
            };

            let possible_weapon_types: Vec<&ItemType> = weapon_types
                .iter()
                .filter(|item_type| item_type_is_for_tags(item_type, &tag))
                .cloned()
                .collect();

            if possible_weapon_types.is_empty() {
                continue;
            }

            let index = rng.gen_range(0..possible_weapon_types.len());
            let weapon_type = match &possible_weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = item_generator_for_level(weapon_type, true, self.danger_level);
            let weapon = generator.generate();

            let hidden_roll = roll_d100(rng, 1, 0);
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

    fn equipped_wearables(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
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
                .filter(|item_type| {
                    type_is_for_wearable(item_type) && item_type_is_for_tags(item_type, &tag)
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
            let hidden_roll = roll_d100(rng, 1, 0);
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

    fn spell_scrolls(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
        let spell_names: Vec<SpellName> = SpellName::iter().collect();
        let index = rng.gen_range(0..spell_names.len());
        let spell_name = spell_names.get(index).unwrap();

        let spell_uses: i32 = if spell_name == &SpellName::Phoenix {
            1
        } else {
            rng.gen_range(1..=6)
        };

        let spell_attack = if matches!(
            spell_name,
            SpellName::RagingFireball | SpellName::ElectricBlast
        ) {
            Some(Attack {
                num_rolls: 2,
                modifier: 0,
            })
        } else if spell_name == &SpellName::Retribution {
            Some(Attack {
                num_rolls: 3,
                modifier: -1,
            })
        } else if spell_name == &SpellName::QuickHeal {
            Some(Attack {
                num_rolls: 1,
                modifier: 0,
            })
        } else if spell_name == &SpellName::Heal {
            Some(Attack {
                num_rolls: 2,
                modifier: 0,
            })
        } else {
            None
        };

        let spell_defense = if spell_name == &SpellName::TinyShield {
            let damage_resistance = rng.gen_range(2..=10);
            Some(Defense { damage_resistance })
        } else {
            None
        };

        let possible_materials = super::utils::materials::possible_materials(&ItemType::Scroll);
        let material = if possible_materials.is_empty() {
            None
        } else {
            let material_index = rng.gen_range(0..possible_materials.len());
            possible_materials.get(material_index).cloned()
        };

        let item = Item {
            id: Uuid::new_v4(),
            name: None,
            item_type: ItemType::Scroll,
            tags: vec![Tag::Consumable, Tag::Teachable],
            descriptors: Vec::new(),
            material,
            attack: None,
            defense: None,
            consumable: Some(Consumable {
                effect: ConsumableEffect {
                    name: ConsumableEffectName::LearnSpell,
                    learn_spell_effect: Some(LearnSpellEffect {
                        spell_name: spell_name.clone(),
                        spell_attack,
                        spell_defense,
                        spell_uses,
                    }),
                },
                uses: 1,
            }),
        };

        vec![CharacterItem {
            item,
            is_hidden: false,
            equipped_location: LocationTag::Packed,
            is_multiple: false,
            at_the_ready: false,
        }]
    }
}

impl Generator<Inventory> for InventoryPrototype {
    fn generate(&self) -> Inventory {
        let mut rng = rand::thread_rng();

        let equipped_weapons = self.equipped_weapons(&mut rng);
        let equipped_wearables = self.equipped_wearables(&mut rng);

        let spell_scrolls = if roll_d100(&mut rng, 1, 0) <= GENERATE_SCROLL_CHANCE {
            self.spell_scrolls(&mut rng)
        } else {
            Vec::new()
        };

        Inventory {
            equipment: equipped_weapons
                .into_iter()
                .chain(equipped_wearables.into_iter())
                .chain(spell_scrolls.into_iter())
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
        ItemType::Crown
        | ItemType::PlateHelmet
        | ItemType::Helm
        | ItemType::BowlerHat
        | ItemType::Fedora
        | ItemType::TopHat => tag.eq(&LocationTag::Head),
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
        ItemType::Scroll => tag.eq(&LocationTag::Packed) | tag.eq(&LocationTag::Pockets),
    }
}
