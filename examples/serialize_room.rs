pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::generators::{generator::Generator, rooms::RoomPrototype};

        let room_prototype = RoomPrototype::build_random(Vec::new(), 1..=3);
        let room = room_prototype.generate();
        let serialized = serde_json::to_string(&room);

        match serialized {
            Ok(it) => println!("{}", it),
            Err(e) => println!("Serialization failed {}", e),
        }
    }
}
