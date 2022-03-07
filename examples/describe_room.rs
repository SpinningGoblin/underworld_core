use underworld_core::{
    components::rooms::room_view::RoomViewArgs,
    generators::{generator::Generator, rooms::RoomPrototype},
    systems::view::room::look_at,
};

pub fn main() {
    let room_prototype = RoomPrototype::build_random();
    let room = room_prototype.generate();
    let view = look_at(&room, RoomViewArgs::default(), true);

    println!("{}", &view);
    println!();
    println!("{}", &view.describe_inhabitants());
}
