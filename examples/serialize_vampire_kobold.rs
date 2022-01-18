use std::env;

use underworld_core::{
    components::{identifier::Identifier, life_modifier::LifeModifier},
    generators::{
        characters::CharacterPrototype, equipped_items::EquippedItemPrototype,
        generator::Generator, weapons::WeaponPrototype, wearables::WearablePrototype,
    },
};

#[cfg(feature = "serialization")]
#[cfg(feature = "json")]
pub fn main() {
    let name_arg = env::args().nth(1);
    let identifier = name_arg.map(|name| Identifier { name });

    let other_item = EquippedItemPrototype {
        generator: Box::new(WeaponPrototype::club()),
        hidden_chance: 0,
        multiple: false,
        equipped_locations: vec!["almost falling from its grip".to_string()],
        equipped_location_chance: 100,
    };

    let kobold_prototype = CharacterPrototype {
        identifier,
        weapon_generators: vec![
            Box::new(WeaponPrototype::dagger()),
            Box::new(WeaponPrototype::long_sword()),
        ],
        equipped_weapon_generators: vec![
            Box::new(EquippedItemPrototype::visible_dagger(100)),
            Box::new(EquippedItemPrototype::visible_long_sword(100)),
            Box::new(EquippedItemPrototype::visible_short_sword(100)),
            Box::new(EquippedItemPrototype::visible_hammer(100)),
            Box::new(other_item),
        ],
        wearable_generators: vec![Box::new(WearablePrototype::armour())],
        equipped_wearable_generators: vec![
            Box::new(EquippedItemPrototype::armour(0, 100)),
            Box::new(EquippedItemPrototype::plate_mail(0, 100)),
        ],
        species: underworld_core::components::species::Species::Kobold,
        life_modifier: Some(LifeModifier::Vampire),
        has_inventory: true,
        num_weapons: 1..2,
        num_equipped_weapons: 1..3,
        num_wearables: 1..3,
        num_equipped_wearables: 1..3,
    };

    let kobold = kobold_prototype.generate();

    let serialized = serde_json::to_string(&kobold);

    match serialized {
        Ok(it) => println!("{}", it),
        Err(e) => println!("Serialization failed {}", e),
    }
}
