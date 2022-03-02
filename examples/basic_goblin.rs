use underworld_core::{
    components::character::CharacterViewArgs,
    generators::{characters::CharacterPrototype, generator::Generator},
};

pub fn main() {
    let goblin_prototype = CharacterPrototype::basic_goblin();
    let goblin = goblin_prototype.generate();
    let view = goblin.look_at(&CharacterViewArgs::default(), true);

    if let Some(inventory) = &view.inventory {
        println!("{}", inventory);
    }
    println!("{}", &view.describe_species());
}
