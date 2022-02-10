use underworld_core::generators::{generator::Generator, rooms::RoomPrototype};

pub fn main() {
    let room_prototype = RoomPrototype::build_random();
    let room = room_prototype.generate();

    println!("{}", &room);
    println!();
    println!("{}", &room.look_at_inhabitants());
}
