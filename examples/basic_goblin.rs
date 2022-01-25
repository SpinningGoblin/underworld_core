use underworld_core::generators::{characters::CharacterPrototype, generator::Generator};

pub fn main() {
    let goblin_prototype = CharacterPrototype::basic_goblin();
    let goblin = goblin_prototype.generate();

    if let Some(inventory) = &goblin.inventory {
        println!("{}", inventory);
    }
    println!("{}", &goblin.describe_species());
}
