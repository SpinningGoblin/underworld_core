use underworld_core::{
    components::character::CharacterViewArgs,
    generators::{characters::CharacterPrototype, generator::Generator},
    systems::view::character::view,
};

pub fn main() {
    let goblin_prototype = CharacterPrototype::basic_goblin();
    let goblin = goblin_prototype.generate();
    let view = view(&goblin, &CharacterViewArgs::default(), true);

    if let Some(inventory) = &view.inventory {
        println!("{}", inventory);
    }
    println!("{}", &view.describe_species());
}
