use enum_iterator::IntoEnumIterator;
use underworld_core::{
    components::{
        character::CharacterViewArgs, items::item_type::ItemType, life_modifier::LifeModifier,
    },
    generators::{
        characters::CharacterPrototype, generator::Generator, inventory::InventoryPrototype,
    },
    systems::view::character::look_at,
};

pub fn main() {
    let inventory_prototype = InventoryPrototype {
        item_types: ItemType::into_enum_iter().collect(),
        num_equipped_weapons: 1..=3,
        num_equipped_wearables: 1..=3,
        num_carried_weapons: 1..=2,
        num_carried_wearables: 1..=2,
        hidden_weapon_chance: 0,
        hidden_wearable_chance: 0,
    };

    let kobold_prototype = CharacterPrototype {
        inventory_generator: Box::new(inventory_prototype),
        species: underworld_core::components::species::Species::Kobold,
        life_modifier: Some(LifeModifier::Vampire),
        has_inventory: true,
    };

    let kobold = kobold_prototype.generate();
    let view = look_at(&kobold, &CharacterViewArgs::default(), true);
    if let Some(inventory) = &view.inventory {
        println!("{}", inventory);
    }
    println!("{}", &view.describe_species());
}
