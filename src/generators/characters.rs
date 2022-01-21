mod prototypes;

use rand::Rng;

use crate::components::{
    character::Character, identifier::Identifier, inventory::Inventory,
    life_modifier::LifeModifier, species::Species,
};

use self::prototypes::{basic_character, overloaded_character, undead_character};

use super::{generator::Generator, stats::StatsPrototype};

pub struct CharacterPrototype {
    pub identifier: Option<Identifier>,
    pub inventory_generator: Box<dyn Generator<Inventory>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
}

impl CharacterPrototype {
    pub fn basic_goblin(identifier: Option<Identifier>) -> Self {
        basic_character(identifier, Species::Goblin)
    }

    pub fn undead_goblin(identifier: Option<Identifier>) -> Self {
        undead_character(identifier, Species::Goblin)
    }

    pub fn overloaded_goblin(identifier: Option<Identifier>) -> Self {
        overloaded_character(identifier, Species::Goblin)
    }

    pub fn basic_kobold(identifier: Option<Identifier>) -> Self {
        basic_character(identifier, Species::Kobold)
    }

    pub fn undead_kobold(identifier: Option<Identifier>) -> Self {
        undead_character(identifier, Species::Kobold)
    }

    pub fn overloaded_kobold(identifier: Option<Identifier>) -> Self {
        overloaded_character(identifier, Species::Kobold)
    }

    pub fn overloaded_character(identifier: Option<Identifier>, species: Species) -> Self {
        overloaded_character(identifier, species)
    }

    pub fn random_species_character(identifier: Option<Identifier>) -> Self {
        let mut rng = rand::thread_rng();
        let all_species = vec![
            Species::Bugbear,
            Species::Goblin,
            Species::Kobold,
            Species::Ogre,
            Species::Orc,
            Species::Unknown,
        ];
        let index = rng.gen_range(0..all_species.len());
        let species = all_species
            .get(index)
            .map(|s| s.clone())
            .unwrap_or(Species::Unknown);

        basic_character(identifier, species)
    }

    pub fn random_species_overloaded(identifier: Option<Identifier>) -> Self {
        let mut rng = rand::thread_rng();
        let all_species = vec![
            Species::Bugbear,
            Species::Goblin,
            Species::Kobold,
            Species::Ogre,
            Species::Orc,
            Species::Unknown,
        ];
        let index = rng.gen_range(0..all_species.len());
        let species = all_species
            .get(index)
            .map(|s| s.clone())
            .unwrap_or(Species::Unknown);

        overloaded_character(identifier, species)
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
