use underworld_core::generators::{generator::Generator, rooms::RoomPrototype};

pub fn main() {
    let npc_names = vec!["Brognar".to_string(), "Zug".to_string()];
    let room_prototype = RoomPrototype::build_random(npc_names, 1..4);
    let room = room_prototype.generate();
    println!("{}", &room);
    println!();
    println!("{}", &room.look_at_inhabitants());

    println!("===============");

    for fixture_position in &room.fixture_positions {
        println!("{}", &fixture_position);
        println!();
        println!("===============");
    }
}
