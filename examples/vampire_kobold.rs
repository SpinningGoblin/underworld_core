use std::env;

use underworld_core::{
    components::{identifier::Identifier, life_modifier::LifeModifier},
    generators::{
        characters::CharacterPrototype, generator::Generator, inventory::InventoryPrototype,
        weapons::WeaponPrototype, wearables::WearablePrototype,
    },
};

pub fn main() {
    let name_arg = env::args().nth(1);
    let identifier = name_arg.map(|name| Identifier { name });

    let inventory_prototype = InventoryPrototype {
        weapon_generators: vec![
            Box::new(WeaponPrototype::dagger()),
            Box::new(WeaponPrototype::long_sword()),
        ],
        wearable_generators: vec![
            Box::new(WearablePrototype::clothing()),
            Box::new(WearablePrototype::cloak()),
            Box::new(WearablePrototype::armour()),
            Box::new(WearablePrototype::plate_mail()),
        ],
        num_equipped_weapons: 1..3,
        num_equipped_wearables: 1..3,
        num_carried_weapons: 1..2,
        num_carried_wearables: 1..2,
        hidden_weapon_chance: 0,
        hidden_wearable_chance: 0,
    };

    let kobold_prototype = CharacterPrototype {
        identifier,
        inventory_generator: Box::new(inventory_prototype),
        species: underworld_core::components::species::Species::Kobold,
        life_modifier: Some(LifeModifier::Vampire),
        has_inventory: true,
    };

    let kobold = kobold_prototype.generate();
    if let Some(inventory) = &kobold.inventory {
        println!("{}", inventory);
    }
    println!("{}", &kobold.describe_species());
    println!("{}", &kobold.describe_name());
}
