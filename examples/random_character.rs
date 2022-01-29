use underworld_core::generators::{
    characters::CharacterPrototype, generator::Generator, non_players::NonPlayerPrototype,
};

pub fn main() {
    let prototype = NonPlayerPrototype {
        name: None,
        character_generator: Box::new(CharacterPrototype::random_species_overloaded()),
    };
    let character = prototype.generate().character;
    if let Some(inventory) = &character.inventory {
        println!("{}", inventory);
    }
    println!("{}", &character.describe_species());
}
