use std::env;

use underworld_core::{
    components::identifier::Identifier,
    generators::{characters::CharacterPrototype, generator::Generator},
};

pub fn main() {
    let name_arg = env::args().nth(1);
    let identifier = name_arg.map(|name| Identifier { name });

    let prototype = CharacterPrototype::random_species_overloaded(identifier);
    let character = prototype.generate();
    if let Some(inventory) = &character.inventory {
        println!("{}", inventory);
    }
    println!("{}", &character.describe_species());
    println!("{}", &character.describe_name());
}
