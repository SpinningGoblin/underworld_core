use std::ops::Range;

use rand::Rng;

use crate::components::{
    equipped_item::{EquipLocationDescriptor, Equippable, EquippedItem},
    inventory::Inventory,
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
    fn equipped_weapons(&self) -> Vec<EquippedItem<Weapon>> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_weapons.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut used_locations: Vec<EquipLocationDescriptor> = Vec::new();
        let mut equipped_weapons: Vec<EquippedItem<Weapon>> = Vec::new();
        for _ in 1..=count {
            let index = rng.gen_range(0..self.weapon_types.len());
            let weapon_type = match &self.weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = WeaponGenerator::for_weapon_type(weapon_type);
            let weapon = generator.generate();
            let possible_locations: Vec<EquipLocationDescriptor> = weapon
                .possible_equip_locations()
                .iter()
                .filter(|location| {
                    if used_locations.is_empty() {
                        true
                    } else {
                        used_locations
                            .iter()
                            .all(|l| !l.unable_to_be_used_with(location))
                    }
                })
                .cloned()
                .collect();

            let range = 0..possible_locations.len();

            let equipped_location = if range.is_empty() {
                EquipLocationDescriptor::None
            } else {
                let location_index = rng.gen_range(range);
                possible_locations
                    .get(location_index)
                    .cloned()
                    .unwrap_or_default()
            };

            if equipped_location != EquipLocationDescriptor::None {
                used_locations.push(equipped_location.clone());
            };

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = weapon.is_multiple();

            equipped_weapons.push(EquippedItem {
                multiple,
                item: weapon,
                hidden: hidden_roll <= self.hidden_weapon_chance,
                equipped_location,
            })
        }

        equipped_weapons
    }

    fn carried_weapons(&self) -> Vec<Weapon> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_carried_weapons.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut equipped_weapons: Vec<Weapon> = Vec::new();
        for _ in 1..=count {
            let index = rng.gen_range(0..self.weapon_types.len());
            let weapon_type = match &self.weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = WeaponGenerator::for_weapon_type(weapon_type);
            let weapon = generator.generate();

            equipped_weapons.push(weapon);
        }

        equipped_weapons
    }

    fn equipped_wearables(&self) -> Vec<EquippedItem<Wearable>> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_wearables.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut used_locations: Vec<EquipLocationDescriptor> = Vec::new();
        let mut equipped_wearables: Vec<EquippedItem<Wearable>> = Vec::new();
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
            let possible_locations: Vec<EquipLocationDescriptor> = wearable
                .possible_equip_locations()
                .iter()
                .filter(|location| {
                    if used_locations.is_empty() {
                        true
                    } else {
                        used_locations
                            .iter()
                            .all(|l| !l.unable_to_be_used_with(location))
                    }
                })
                .cloned()
                .collect();

            let range = 0..possible_locations.len();

            let equipped_location = if range.is_empty() {
                EquipLocationDescriptor::None
            } else {
                let location_index = rng.gen_range(range);
                possible_locations
                    .get(location_index)
                    .cloned()
                    .unwrap_or_default()
            };

            if equipped_location != EquipLocationDescriptor::None {
                used_locations.push(equipped_location.clone());
            };

            let hidden_roll: usize = rng.gen_range(0..=100);
            let multiple = wearable.is_multiple();

            equipped_wearables.push(EquippedItem {
                multiple,
                item: wearable,
                hidden: hidden_roll <= self.hidden_wearable_chance,
                equipped_location,
            })
        }

        equipped_wearables
    }

    fn carried_wearables(&self) -> Vec<Wearable> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(self.num_equipped_wearables.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut wearables: Vec<Wearable> = Vec::new();
        for _ in 1..=count {
            let index = rng.gen_range(0..self.wearable_types.len());
            let wearable_type = match &self.wearable_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = WearableGenerator::for_wearable_type(wearable_type);
            let wearable = generator.generate();
            wearables.push(wearable);
        }

        wearables
    }
}

impl Generator<Inventory> for InventoryPrototype {
    fn generate(&self) -> Inventory {
        let equipped_weapons = self.equipped_weapons();
        let equipped_wearables = self.equipped_wearables();
        let carried_weapons = self.carried_weapons();
        let carried_wearables = self.carried_wearables();

        Inventory {
            equipped_weapons,
            equipped_wearables,
            carried_weapons,
            carried_wearables,
        }
    }
}
