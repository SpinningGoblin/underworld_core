pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::generators::{generator::Generator, rooms::RoomPrototype};

        let npc_names: Vec<String> =
            vec!["Zug".to_string(), "Borok".to_string(), "Zombie".to_string()];

        let room_prototype = RoomPrototype::build_random(npc_names);
        let room = room_prototype.generate();
        let serialized = serde_json::to_string(&room);

        match serialized {
            Ok(it) => println!("{}", it),
            Err(e) => println!("Serialization failed {}", e),
        }
    }
}
