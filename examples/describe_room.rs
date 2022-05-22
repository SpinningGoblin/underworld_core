use std::collections::HashMap;

use underworld_core::{
    generators::{generator::Generator, rooms::random_room_generator},
    systems::view::room::view,
};

pub fn main() {
    let room_generator = random_room_generator(None);
    let room = room_generator.generate();
    let view = view(&room, HashMap::new(), HashMap::new(), true);

    println!("{}", &view);
    println!();
    println!("{}", &view.describe_inhabitants());
}
