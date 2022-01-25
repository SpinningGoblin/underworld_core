use std::ops::Range;

use rand::Rng;

use crate::{
    components::{
        life_modifier::LifeModifier, species::Species, weapon::Weapon, wearable::Wearable,
    },
    generators::{
        generator::Generator, inventory::InventoryPrototype, weapons::WeaponPrototype,
        wearables::WearablePrototype,
    },
};

use super::CharacterPrototype;

struct CharacterArgs {
    num_equipped_weapons: Range<usize>,
    num_equipped_wearables: Range<usize>,
    num_carried_weapons: Range<usize>,
    num_carried_wearables: Range<usize>,
    species: Species,
    weapon_generators: Vec<Box<dyn Generator<Weapon>>>,
    wearable_generators: Vec<Box<dyn Generator<Wearable>>>,
    life_modifier: Option<LifeModifier>,
    has_inventory: bool,
}

pub fn basic_character(species: Species) -> CharacterPrototype {
    let args = CharacterArgs {
        species,
        num_equipped_weapons: 1..3,
        num_equipped_wearables: 1..3,
        num_carried_weapons: 0..2,
        num_carried_wearables: 0..2,
        weapon_generators: WeaponPrototype::all(),
        wearable_generators: WearablePrototype::all(),
        life_modifier: None,
        has_inventory: true,
    };

    character(args)
}

pub fn overloaded_character(species: Species) -> CharacterPrototype {
    let args = CharacterArgs {
        species,
        num_equipped_weapons: 2..4,
        num_equipped_wearables: 2..5,
        num_carried_weapons: 2..6,
        num_carried_wearables: 2..6,
        weapon_generators: WeaponPrototype::all(),
        wearable_generators: WearablePrototype::all(),
        life_modifier: None,
        has_inventory: true,
    };

    character(args)
}

pub fn undead_character(species: Species) -> CharacterPrototype {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..2);
    let life_modifier = vec![LifeModifier::Vampire, LifeModifier::Zombie]
        .get(index)
        .cloned();

    let args = CharacterArgs {
        life_modifier,
        species,
        num_equipped_weapons: 1..3,
        num_equipped_wearables: 1..3,
        num_carried_weapons: 0..2,
        num_carried_wearables: 0..2,
        weapon_generators: WeaponPrototype::all(),
        wearable_generators: WearablePrototype::all(),
        has_inventory: true,
    };

    character(args)
}

fn character(args: CharacterArgs) -> CharacterPrototype {
    let inventory_prototype = InventoryPrototype {
        weapon_generators: args.weapon_generators,
        wearable_generators: args.wearable_generators,
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
