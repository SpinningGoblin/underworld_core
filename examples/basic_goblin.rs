use std::env;

use underworld_core::{
    components::name::Name,
    generators::{characters::CharacterPrototype, generator::Generator},
};

pub fn main() {
    let name_arg = env::args().nth(1);
    let name = name_arg.map(Name);
    let goblin_prototype = CharacterPrototype::basic_goblin(name);
    let goblin = goblin_prototype.generate();

    if let Some(inventory) = &goblin.inventory {
        println!("{}", inventory);
    }
    println!("{}", &goblin.describe_species());
    println!("{}", &goblin.describe_name());
}
