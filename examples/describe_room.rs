use underworld_core::{
    components::rooms::room_view::RoomViewArgs,
    generators::{rooms::random_room_generator, generator::Generator},
    systems::view::room::look_at,
};

pub fn main() {
    let room_generator = random_room_generator();
    let room = room_generator.generate();
    let view = look_at(&room, RoomViewArgs::default(), true);

    println!("{}", &view);
    println!();
    println!("{}", &view.describe_inhabitants());
}
