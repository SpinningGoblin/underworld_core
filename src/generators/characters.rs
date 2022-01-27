mod prototypes;

use rand::Rng;

use crate::components::{
    character::Character, inventory::Inventory, life_modifier::LifeModifier, species::Species,
};

use self::prototypes::{basic_character, overloaded_character, undead_character};

use super::{generator::Generator, stats::StatsPrototype};

pub struct CharacterPrototype {
    pub inventory_generator: Box<dyn Generator<Inventory>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
}

impl CharacterPrototype {
    pub fn basic_goblin() -> Self {
        basic_character(Species::Goblin)
    }

    pub fn undead_goblin() -> Self {
        undead_character(Species::Goblin)
    }

    pub fn overloaded_goblin() -> Self {
        overloaded_character(Species::Goblin)
    }

    pub fn basic_kobold() -> Self {
        basic_character(Species::Kobold)
    }

    pub fn undead_kobold() -> Self {
        undead_character(Species::Kobold)
    }

    pub fn overloaded_kobold() -> Self {
        overloaded_character(Species::Kobold)
    }

    pub fn overloaded_character(species: Species) -> Self {
        overloaded_character(species)
    }

    pub fn random_species_character() -> Self {
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
        let species = all_species.get(index).cloned().unwrap_or(Species::Unknown);

        basic_character(species)
    }

    pub fn random_species_overloaded() -> Self {
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
        let species = all_species.get(index).cloned().unwrap_or(Species::Unknown);

        overloaded_character(species)
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
            species: self.species.clone(),
            life_modifier: self.life_modifier.clone(),
        }
    }
}

#[cfg(test)]
mod goblin_tests {
    use crate::generators::generator::Generator;

    use super::CharacterPrototype;

    #[test]
    fn basic_goblin() {
        let prototype = CharacterPrototype::basic_goblin();
        let goblin = prototype.generate();
        assert!(goblin.inventory.is_some());
        println!("{}", goblin.inventory.unwrap());
    }
}
