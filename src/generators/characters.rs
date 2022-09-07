use rand::Rng;
use strum::IntoEnumIterator;

use crate::components::{
    spells::SpellMemory, Character, Effects, Inventory, LifeModifier, Species,
};

use super::{
    generator::Generator, stats::build_default_health_rolls_for_danger_level,
    InventoryGeneratorBuilder,
};

struct CharacterPrototype {
    pub inventory_gen_builder: InventoryGeneratorBuilder,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
    pub danger_level: u32,
}

#[derive(Default, Clone)]
pub struct CharacterGeneratorBuilder {
    inventory_generator_builder: Option<InventoryGeneratorBuilder>,
    species: Option<Species>,
    life_modifier: Option<LifeModifier>,
    has_inventory: Option<bool>,
    danger_level: Option<u32>,
}

impl CharacterGeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inventory_generator_builder(&mut self, builder: InventoryGeneratorBuilder) -> &mut Self {
        self.inventory_generator_builder = Some(builder);

        self
    }

    pub fn species(&mut self, species: Species) -> &mut Self {
        self.species = Some(species);

        self
    }

    pub fn life_modifier(&mut self, life_modifier: LifeModifier) -> &mut Self {
        self.life_modifier = Some(life_modifier);

        self
    }

    pub fn has_inventory(&mut self, has_inventory: bool) -> &mut Self {
        self.has_inventory = Some(has_inventory);

        self
    }

    pub fn danger_level(&mut self, danger_level: u32) -> &mut Self {
        self.danger_level = Some(danger_level);

        self
    }

    pub fn build(&self) -> impl Generator<Character> {
        let mut rng = rand::thread_rng();
        let danger_level = self.danger_level.unwrap_or(1);

        let inventory_gen_builder = match &self.inventory_generator_builder {
            Some(builder) => builder.to_owned(),
            None => InventoryGeneratorBuilder::new()
                .danger_level(danger_level)
                .to_owned(),
        };

        let species = match &self.species {
            Some(it) => it.to_owned(),
            None => {
                let all: Vec<Species> = Species::iter().collect();
                let index = rng.gen_range(0..all.len());
                all.get(index).unwrap().to_owned()
            }
        };

        CharacterPrototype {
            inventory_gen_builder,
            species,
            life_modifier: self.life_modifier,
            has_inventory: self.has_inventory.unwrap_or(true),
            danger_level,
        }
    }
}

impl Generator<Character> for CharacterPrototype {
    fn generate(&self) -> Character {
        let inventory = if self.has_inventory {
            self.inventory_gen_builder.build().generate()
        } else {
            Inventory::default()
        };

        let stats_generator =
            build_default_health_rolls_for_danger_level(&self.species, self.danger_level, true);
        let stats = stats_generator.generate();

        Character {
            stats,
            inventory,
            species: self.species,
            life_modifier: self.life_modifier,
            current_effects: Effects::default(),
            spell_memory: SpellMemory::default(),
        }
    }
}
