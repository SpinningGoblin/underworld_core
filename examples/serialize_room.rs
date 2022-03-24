pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::generators::{generator::Generator, rooms::random_room_generator};

        let room_prototype = random_room_generator(None);
        let room = room_prototype.generate();
        let serialized = serde_json::to_string(&room);

        match serialized {
            Ok(it) => println!("{}", it),
            Err(e) => println!("Serialization failed {}", e),
        }
    }
}
