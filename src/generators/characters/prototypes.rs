use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    components::{items::item_type::ItemType, life_modifier::LifeModifier, species::Species},
    generators::inventory::InventoryPrototype,
};

use super::CharacterPrototype;

struct CharacterArgs {
    num_equipped_weapons: RangeInclusive<usize>,
    num_equipped_wearables: RangeInclusive<usize>,
    num_carried_weapons: RangeInclusive<usize>,
    num_carried_wearables: RangeInclusive<usize>,
    species: Species,
    item_types: Vec<ItemType>,
    life_modifier: Option<LifeModifier>,
    has_inventory: bool,
}

pub fn basic_character(species: Species) -> CharacterPrototype {
    let args = CharacterArgs {
        species,
        num_equipped_weapons: 1..=2,
        num_equipped_wearables: 1..=2,
        num_carried_weapons: 0..=1,
        num_carried_wearables: 0..=1,
        item_types: ItemType::iter().collect(),
        life_modifier: None,
        has_inventory: true,
    };

    character(args)
}

pub fn overloaded_character(species: Species) -> CharacterPrototype {
    let args = CharacterArgs {
        species,
        num_equipped_weapons: 1..=2,
        num_equipped_wearables: 2..=3,
        num_carried_weapons: 2..=5,
        num_carried_wearables: 2..=5,
        item_types: ItemType::iter().collect(),
        life_modifier: None,
        has_inventory: true,
    };

    character(args)
}

pub fn undead_character(species: Species) -> CharacterPrototype {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..=1);
    let life_modifier = vec![LifeModifier::Vampire, LifeModifier::Zombie]
        .get(index)
        .cloned();

    let args = CharacterArgs {
        life_modifier,
        species,
        num_equipped_weapons: 1..=2,
        num_equipped_wearables: 1..=2,
        num_carried_weapons: 0..=1,
        num_carried_wearables: 0..=1,
        item_types: ItemType::iter().collect(),
        has_inventory: true,
    };

    character(args)
}

fn character(args: CharacterArgs) -> CharacterPrototype {
    let inventory_prototype = InventoryPrototype {
        item_types: args.item_types,
        num_equipped_weapons: args.num_equipped_weapons,
        num_equipped_wearables: args.num_equipped_wearables,
        num_carried_weapons: args.num_carried_weapons,
        num_carried_wearables: args.num_carried_wearables,
        hidden_weapon_chance: 0,
        hidden_wearable_chance: 0,
    };

    CharacterPrototype {
        inventory_generator: Box::new(inventory_prototype),
        species: args.species,
        life_modifier: args.life_modifier,
        has_inventory: args.has_inventory,
    }
}
