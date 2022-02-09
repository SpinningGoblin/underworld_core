use std::ops::RangeInclusive;

use rand::Rng;

use crate::{
    components::{
        life_modifier::LifeModifier, species::Species, weapons::weapon_type::WeaponType,
        wearables::wearable_type::WearableType,
    },
    generators::inventory::InventoryPrototype,
};

use super::CharacterPrototype;

struct CharacterArgs {
    num_equipped_weapons: RangeInclusive<usize>,
    num_equipped_wearables: RangeInclusive<usize>,
    num_carried_weapons: RangeInclusive<usize>,
    num_carried_wearables: RangeInclusive<usize>,
    species: Species,
    weapon_types: Vec<WeaponType>,
    wearable_types: Vec<WearableType>,
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
        weapon_types: WeaponType::all(),
        wearable_types: WearableType::all(),
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
        weapon_types: WeaponType::all(),
        wearable_types: WearableType::all(),
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
        weapon_types: WeaponType::all(),
        wearable_types: WearableType::all(),
        has_inventory: true,
    };

    character(args)
}

fn character(args: CharacterArgs) -> CharacterPrototype {
    let inventory_prototype = InventoryPrototype {
        weapon_types: args.weapon_types,
        wearable_types: args.wearable_types,
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
