use std::ops::Range;

use rand::Rng;

use crate::components::{
    equipped_item::{Equippable, EquippedItem, EquippedLocation},
    inventory::Inventory,
    weapon::Weapon,
    wearable::Wearable,
};

use super::generator::Generator;

pub struct InventoryPrototype {
    pub weapon_generators: Vec<Box<dyn Generator<Weapon>>>,
    pub wearable_generators: Vec<Box<dyn Generator<Wearable>>>,
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

        let mut used_locations: Vec<EquippedLocation> = Vec::new();
        let mut equipped_weapons: Vec<EquippedItem<Weapon>> = Vec::new();
        for _ in 0..count {
            let index = rng.gen_range(0..self.weapon_generators.len());
            let generator = match &self.weapon_generators.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let weapon = generator.generate();
            let possible_locations: Vec<EquippedLocation> = weapon
                .possible_equip_locations()
                .iter()
                .filter(|location| {
                    if used_locations.is_empty() {
                        true
                    } else {
                        used_locations
                            .iter()
                            .any(|l| !l.unable_to_be_used_with(location))
                    }
                })
                .cloned()
                .collect();

            let range = 0..possible_locations.len();

            let equipped_location = if range.is_empty() {
                EquippedLocation::None
            } else {
                let location_index = rng.gen_range(range);
                possible_locations
                    .get(location_index)
                    .cloned()
                    .unwrap_or_default()
            };

            if equipped_location != EquippedLocation::None {
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
        for _ in 0..count {
            let index = rng.gen_range(0..self.weapon_generators.len());
            let generator = match &self.weapon_generators.get(index) {
                Some(it) => *it,
                None => continue,
            };
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

        let mut used_locations: Vec<EquippedLocation> = Vec::new();
        let mut equipped_wearables: Vec<EquippedItem<Wearable>> = Vec::new();
        for _ in 0..count {
            let index = rng.gen_range(0..self.wearable_generators.len());
            let generator = match &self.wearable_generators.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let wearable = generator.generate();
            let possible_locations: Vec<EquippedLocation> = wearable
                .possible_equip_locations()
                .iter()
                .filter(|location| {
                    if used_locations.is_empty() {
                        true
                    } else {
                        used_locations
                            .iter()
                            .any(|l| !l.unable_to_be_used_with(location))
                    }
                })
                .cloned()
                .collect();

            let range = 0..possible_locations.len();

            let equipped_location = if range.is_empty() {
                EquippedLocation::None
            } else {
                let location_index = rng.gen_range(range);
                possible_locations
                    .get(location_index)
                    .cloned()
                    .unwrap_or_default()
            };

            if equipped_location != EquippedLocation::None {
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
        for _ in 0..count {
            let index = rng.gen_range(0..self.wearable_generators.len());
            let generator = match &self.wearable_generators.get(index) {
                Some(it) => *it,
                None => continue,
            };
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
