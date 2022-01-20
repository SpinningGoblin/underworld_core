use crate::components::{
    character::Character, identifier::Identifier, inventory::Inventory,
    life_modifier::LifeModifier, species::Species,
};

use super::{
    generator::Generator, inventory::InventoryPrototype, stats::StatsPrototype,
    weapons::WeaponPrototype, wearables::WearablePrototype,
};

pub struct CharacterPrototype {
    pub identifier: Option<Identifier>,
    pub inventory_generator: Box<dyn Generator<Inventory>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
}

impl CharacterPrototype {
    pub fn basic_goblin(name: Option<Identifier>) -> Self {
        let inventory_prototype = InventoryPrototype {
            weapon_generators: vec![
                Box::new(WeaponPrototype::dagger()),
                Box::new(WeaponPrototype::club()),
            ],
            wearable_generators: vec![
                Box::new(WearablePrototype::clothing()),
                Box::new(WearablePrototype::cloak()),
                Box::new(WearablePrototype::armour()),
                Box::new(WearablePrototype::shackles()),
            ],
            num_equipped_weapons: 1..3,
            num_equipped_wearables: 1..3,
            num_carried_weapons: 0..2,
            num_carried_wearables: 0..2,
            hidden_weapon_chance: 0,
            hidden_wearable_chance: 0,
        };

        Self {
            identifier: name,
            inventory_generator: Box::new(inventory_prototype),
            species: Species::Goblin,
            life_modifier: None,
            has_inventory: true,
        }
    }
}

impl Generator<Character> for CharacterPrototype {
    fn generate(&self) -> Character {
        let inventory = if self.has_inventory {
            Some(self.inventory_generator.generate())
        } else {
            None
        };

        let stats_generator = StatsPrototype::from(&self.species);
        let stats = stats_generator.generate();

        Character {
            stats,
            inventory,
            name: self.identifier.clone(),
            species: self.species.clone(),
            life_modifier: self.life_modifier.clone(),
        }
    }
}

#[cfg(test)]
mod goblin_tests {
    use crate::{components::identifier::Identifier, generators::generator::Generator};

    use super::CharacterPrototype;

    #[test]
    fn basic_goblin() {
        let prototype = CharacterPrototype::basic_goblin(Some(Identifier {
            name: "gerblin".to_string(),
        }));
        let goblin = prototype.generate();
        assert_eq!("gerblin", goblin.name.unwrap().name);
        assert!(goblin.inventory.is_some());
        println!("{}", goblin.inventory.unwrap());
    }
}
