use std::ops::Range;

use rand::Rng;

use crate::components::{
    character::Character, equipped_item::EquippedItem, inventory::Inventory,
    life_modifier::LifeModifier, name::Name, species::Species, weapon::Weapon, wearable::Wearable,
};

use super::{
    equipped_items::EquippedItemPrototype, generator::Generator, stats::StatsPrototype,
    weapons::WeaponPrototype,
};

pub struct CharacterPrototype {
    pub name: Option<Name>,
    pub weapon_generators: Vec<Box<dyn Generator<Weapon>>>,
    pub equipped_weapon_generators: Vec<Box<dyn Generator<EquippedItem<Weapon>>>>,
    pub wearable_generators: Vec<Box<dyn Generator<Wearable>>>,
    pub equipped_wearable_generators: Vec<Box<dyn Generator<EquippedItem<Wearable>>>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
    pub num_weapons: Range<usize>,
    pub num_equipped_weapons: Range<usize>,
    pub num_wearables: Range<usize>,
    pub num_equipped_wearables: Range<usize>,
}

impl CharacterPrototype {
    pub fn basic_goblin(name: Option<Name>) -> Self {
        Self {
            name,
            weapon_generators: vec![
                Box::new(WeaponPrototype::dagger()),
                Box::new(WeaponPrototype::club()),
            ],
            equipped_weapon_generators: vec![
                Box::new(EquippedItemPrototype::visible_dagger(75)),
                Box::new(EquippedItemPrototype::visible_short_sword(75)),
                Box::new(EquippedItemPrototype::visible_club(75)),
            ],
            wearable_generators: Vec::new(),
            equipped_wearable_generators: vec![
                Box::new(EquippedItemPrototype::clothing(0, 75)),
                Box::new(EquippedItemPrototype::armour(0, 75)),
                Box::new(EquippedItemPrototype::cloak(0, 75)),
                Box::new(EquippedItemPrototype::shackles(0, 75)),
            ],
            species: Species::Goblin,
            life_modifier: None,
            has_inventory: true,
            num_weapons: 0..2,
            num_equipped_weapons: 1..2,
            num_wearables: 0..1,
            num_equipped_wearables: 1..3,
        }
    }

    fn equipped_weapons(&self) -> Vec<EquippedItem<Weapon>> {
        let mut rng = rand::thread_rng();
        let mut equipped_weapons: Vec<EquippedItem<Weapon>> = Vec::new();

        if self.num_equipped_weapons.is_empty() {
            return equipped_weapons;
        }

        if !self.equipped_weapon_generators.is_empty() {
            let num_weapons: usize = rng.gen_range(self.num_equipped_weapons.clone());

            if num_weapons == 0 {
                return equipped_weapons;
            }

            for _ in 0..num_weapons {
                let index = rng.gen_range(0..self.equipped_weapon_generators.len());
                if let Some(generator) = self.equipped_weapon_generators.get(index) {
                    let equipped_weapon = generator.generate().clone();
                    equipped_weapons.push(equipped_weapon);
                }
            }
        }

        equipped_weapons
    }

    fn equipped_wearables(&self) -> Vec<EquippedItem<Wearable>> {
        let mut rng = rand::thread_rng();
        let mut equipped_wearables: Vec<EquippedItem<Wearable>> = Vec::new();

        if self.num_equipped_wearables.is_empty() {
            return equipped_wearables;
        }

        if !self.equipped_wearable_generators.is_empty() {
            let mut indices: Vec<usize> = (0..self.equipped_wearable_generators.len()).collect();
            let num_wearables: usize = rng.gen_range(self.num_equipped_wearables.clone());

            if num_wearables == 0 {
                return equipped_wearables;
            }

            for _ in 0..num_wearables {
                if indices.is_empty() {
                    break;
                }

                let index = rng.gen_range(0..indices.len());
                let selected_generator_index = indices.remove(index);
                if let Some(generator) = &self
                    .equipped_wearable_generators
                    .get(selected_generator_index)
                {
                    let equipped_wearable = generator.generate().clone();
                    equipped_wearables.push(equipped_wearable);
                }
            }
        }

        equipped_wearables
    }

    fn carried_weapons(&self) -> Vec<Weapon> {
        let mut rng = rand::thread_rng();
        let mut weapons: Vec<Weapon> = Vec::new();

        if self.num_weapons.is_empty() {
            return weapons;
        }

        if !self.weapon_generators.is_empty() {
            let num_weapons: usize = rng.gen_range(self.num_weapons.clone());
            if num_weapons == 0 {
                return weapons;
            }

            for _ in 0..num_weapons {
                let index = rng.gen_range(0..self.weapon_generators.len());
                if let Some(generator) = &self.weapon_generators.get(index) {
                    let weapon = generator.generate().clone();
                    weapons.push(weapon);
                }
            }
        }

        weapons
    }

    fn carried_wearables(&self) -> Vec<Wearable> {
        let mut rng = rand::thread_rng();
        let mut wearables: Vec<Wearable> = Vec::new();

        if self.num_wearables.is_empty() {
            return wearables;
        }

        if !self.wearable_generators.is_empty() {
            let num_wearables: usize = rng.gen_range(self.num_wearables.clone());
            if num_wearables == 0 {
                return wearables;
            }

            for _ in 0..num_wearables {
                let index = rng.gen_range(0..self.wearable_generators.len());
                if let Some(generator) = self.wearable_generators.get(index) {
                    let wearable = generator.generate().clone();
                    wearables.push(wearable);
                }
            }
        }

        wearables
    }
}

impl Generator<Character> for CharacterPrototype {
    fn generate(&self) -> Character {
        let equipped_weapons = self.equipped_weapons();
        let equipped_wearables = self.equipped_wearables();
        let carried_weapons = self.carried_weapons();
        let carried_wearables = self.carried_wearables();

        let inventory = if self.has_inventory {
            Some(Inventory {
                equipped_weapons,
                equipped_wearables,
                carried_weapons,
                carried_wearables,
            })
        } else {
            None
        };

        let stats_generator = StatsPrototype::from(&self.species);
        let stats = stats_generator.generate();

        Character {
            stats,
            inventory,
            name: self.name.clone(),
            species: self.species.clone(),
            life_modifier: self.life_modifier.clone(),
        }
    }
}

#[cfg(test)]
mod goblin_tests {
    use crate::{components::name::Name, generators::generator::Generator};

    use super::CharacterPrototype;

    #[test]
    fn basic_goblin() {
        let prototype = CharacterPrototype::basic_goblin(Some(Name("gerblin".to_string())));
        let goblin = prototype.generate();
        assert_eq!("gerblin", goblin.name.unwrap().0);
        assert!(goblin.inventory.is_some());
        println!("{}", goblin.inventory.unwrap());
    }
}
