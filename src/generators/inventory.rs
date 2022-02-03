use std::ops::Range;

use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::components::{
    character_item::CharacterItem,
    equipment::{location_descriptor::LocationDescriptor, Equipment},
    inventory::Inventory,
    item_tag::TaggedItem,
    object::Object,
    weapons::{weapon::Weapon, weapon_type::WeaponType},
    wearables::{wearable::Wearable, wearable_type::WearableType},
};

use super::{generator::Generator, weapons::WeaponGenerator, wearables::WearableGenerator};

pub struct InventoryPrototype {
    pub weapon_types: Vec<WeaponType>,
    pub wearable_types: Vec<WearableType>,
    pub num_equipped_weapons: Range<usize>,
    pub num_equipped_wearables: Range<usize>,
    pub num_carried_weapons: Range<usize>,
    pub num_carried_wearables: Range<usize>,
    pub hidden_weapon_chance: usize,
    pub hidden_wearable_chance: usize,
}

impl InventoryPrototype {
    fn equipped_weapons(&self) -> Vec<CharacterItem<Weapon>> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_weapons.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut used_descriptors: Vec<LocationDescriptor> = Vec::new();
        let mut equipped_weapons: Vec<CharacterItem<Weapon>> = Vec::new();
        for _ in 1..=count {
            let index = rng.gen_range(0..self.weapon_types.len());
            let weapon_type = match &self.weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = WeaponGenerator::for_weapon_type(weapon_type);
            let weapon = generator.generate();

            let possibilities: Vec<LocationDescriptor> = LocationDescriptor::into_enum_iter()
                .filter(|descriptor| {
                    descriptor.matches_any_location_tags(weapon.character_location_tags())
                        && descriptor.matches_any_item_tags(weapon.weapon_type.tags())
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

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = weapon.is_multiple();

            equipped_weapons.push(CharacterItem {
                is_multiple: multiple,
                item: weapon,
                is_hidden: hidden_roll <= self.hidden_weapon_chance,
                location_descriptor: equipped_location,
            })
        }

        equipped_weapons
    }

    fn equipped_wearables(&self) -> Vec<CharacterItem<Wearable>> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_wearables.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut used_descriptors: Vec<LocationDescriptor> = Vec::new();
        let mut equipped_wearables: Vec<CharacterItem<Wearable>> = Vec::new();
        let mut used_types: Vec<WearableType> = Vec::new();
        for _ in 1..=count {
            let possible_types: Vec<WearableType> = self
                .wearable_types
                .iter()
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
            let wearable_type = possible_types.get(index).unwrap();
            used_types.push(wearable_type.clone());
            let generator = WearableGenerator::for_wearable_type(wearable_type);
            let wearable = generator.generate();

            let possibilities: Vec<LocationDescriptor> = LocationDescriptor::into_enum_iter()
                .filter(|descriptor| {
                    descriptor.matches_any_location_tags(wearable.character_location_tags())
                        && descriptor.matches_any_item_tags(wearable.wearable_type.tags())
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

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = wearable.is_multiple();

            equipped_wearables.push(CharacterItem {
                is_multiple: multiple,
                item: wearable,
                is_hidden: hidden_roll <= self.hidden_wearable_chance,
                location_descriptor: equipped_location,
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
            weapons: equipped_weapons,
            wearables: equipped_wearables,
        }
    }
}
