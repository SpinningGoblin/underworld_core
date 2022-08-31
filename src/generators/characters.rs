use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;

use crate::components::{
    items::ItemType, spells::SpellMemory, Character, Effects, Inventory, LifeModifier, Species,
};

use super::{
    generator::Generator, inventory::InventoryPrototype,
    stats::build_default_health_rolls_for_danger_level,
};

pub struct CharacterPrototype {
    pub inventory_generator: Box<dyn Generator<Inventory>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
    pub danger_level: u32,
}

struct CharacterArgs {
    num_equipped_weapons: RangeInclusive<usize>,
    num_equipped_wearables: RangeInclusive<usize>,
    species: Species,
    item_types: Vec<ItemType>,
    life_modifier: Option<LifeModifier>,
    has_inventory: bool,
}

pub fn random_character_generator() -> impl Generator<Character> {
    random_species_character()
}

pub fn species_character_generator(species: Species) -> impl Generator<Character> {
    basic_character(species)
}

fn basic_character(species: Species) -> CharacterPrototype {
    let args = CharacterArgs {
        species,
        num_equipped_weapons: 1..=2,
        num_equipped_wearables: 1..=2,
        item_types: ItemType::iter().collect(),
        life_modifier: None,
        has_inventory: true,
    };

    character(args)
}

fn character(args: CharacterArgs) -> CharacterPrototype {
    let inventory_prototype = InventoryPrototype {
        item_types: args.item_types,
        num_equipped_weapons: args.num_equipped_weapons,
        num_equipped_wearables: args.num_equipped_wearables,
        danger_level: 1,
    };

    CharacterPrototype {
        inventory_generator: Box::new(inventory_prototype),
        species: args.species,
        life_modifier: args.life_modifier,
        has_inventory: args.has_inventory,
        danger_level: 1,
    }
}

fn random_species_character() -> CharacterPrototype {
    let mut rng = rand::thread_rng();
    let all_species: Vec<Species> = Species::iter().collect();
    let index = rng.gen_range(0..all_species.len());
    let species = all_species.get(index).cloned().unwrap_or(Species::Shadow);

    basic_character(species)
}

impl Generator<Character> for CharacterPrototype {
    fn generate(&self) -> Character {
        let inventory = if self.has_inventory {
            self.inventory_generator.generate()
        } else {
            Inventory::default()
        };

        let stats_generator =
            build_default_health_rolls_for_danger_level(&self.species, self.danger_level, true);
        let stats = stats_generator.generate();

        Character {
            stats,
            inventory,
            species: self.species.clone(),
            life_modifier: self.life_modifier.clone(),
            current_effects: Effects::default(),
            spell_memory: SpellMemory::default(),
        }
    }
}
