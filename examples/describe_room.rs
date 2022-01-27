use underworld_core::generators::{generator::Generator, rooms::RoomPrototype};

pub fn main() {
    let npc_names: Vec<String> = vec!["Zug".to_string(), "Borok".to_string(), "Zombie".to_string()];
    let room_prototype = RoomPrototype::build_random(npc_names);
    let room = room_prototype.generate();
    println!("{}", room);
}
