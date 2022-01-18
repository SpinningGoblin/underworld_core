use std::env;

use underworld_core::{
    components::identifier::Identifier,
    generators::{characters::CharacterPrototype, generator::Generator},
};

pub fn main() {
    let name_arg = env::args().nth(1);
    let identifier = name_arg.map(|name| Identifier { name });
    let goblin_prototype = CharacterPrototype::basic_goblin(identifier);
    let goblin = goblin_prototype.generate();

    if let Some(inventory) = &goblin.inventory {
        println!("{}", inventory);
    }
    println!("{}", &goblin.describe_species());
    println!("{}", &goblin.describe_name());
}
