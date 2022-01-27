pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::{
            components::life_modifier::LifeModifier,
            generators::{
                characters::CharacterPrototype, generator::Generator,
                inventory::InventoryPrototype, non_players::NonPlayerPrototype,
                weapons::WeaponPrototype, wearables::WearablePrototype,
            },
        };

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
            inventory_generator: Box::new(inventory_prototype),
            species: underworld_core::components::species::Species::Kobold,
            life_modifier: Some(LifeModifier::Vampire),
            has_inventory: true,
        };

        let npc_prototype = NonPlayerPrototype {
            name: None,
            character_generator: Box::new(kobold_prototype),
        };

        let kobold = npc_prototype.generate();
        let serialized = serde_json::to_string(&kobold);

        match serialized {
            Ok(it) => println!("{}", it),
            Err(e) => println!("Serialization failed {}", e),
        }
    }
}
