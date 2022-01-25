use underworld_core::generators::{characters::CharacterPrototype, generator::Generator};

pub fn main() {
    let prototype = CharacterPrototype::random_species_overloaded();
    let character = prototype.generate();
    if let Some(inventory) = &character.inventory {
        println!("{}", inventory);
    }
    println!("{}", &character.describe_species());
}
