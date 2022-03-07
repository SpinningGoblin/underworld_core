use underworld_core::{
    components::character::CharacterViewArgs,
    generators::{characters::CharacterPrototype, generator::Generator},
    systems::view::character::look_at,
};

pub fn main() {
    let goblin_prototype = CharacterPrototype::basic_goblin();
    let goblin = goblin_prototype.generate();
    let view = look_at(&goblin, &CharacterViewArgs::default(), true);

    if let Some(inventory) = &view.inventory {
        println!("{}", inventory);
    }
    println!("{}", &view.describe_species());
}
