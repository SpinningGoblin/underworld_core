use std::ops::Range;

use rand::Rng;

use crate::components::{
    character::Character,
    dimensions::Dimensions,
    identifier::Identifier,
    non_player::NonPlayer,
    room::{Room, RoomDescriptor, RoomType},
};

use super::{
    characters::CharacterPrototype, dimensions::DimensionsPrototype, generator::Generator,
    non_players::NonPlayerPrototype,
};

pub struct RoomPrototype {
    pub non_player_generator: Box<dyn Generator<NonPlayer>>,
    pub num_non_players: Range<usize>,
    pub num_descriptors: Range<usize>,
    pub room_type: RoomType,
    pub dimensions_generator: Box<dyn Generator<Dimensions>>,
    pub possible_descriptors: Vec<RoomDescriptor>,
}

impl Generator<Room> for RoomPrototype {
    fn generate(&self) -> Room {
        let mut rng = rand::thread_rng();
        let num_non_players = rng.gen_range(self.num_non_players.clone());

        let mut non_players: Vec<NonPlayer> = Vec::new();
        let non_player_range = 0..=num_non_players;

        if !non_player_range.is_empty() {
            for _ in 0..num_non_players {
                let non_player = self.non_player_generator.generate();
                non_players.push(non_player);
            }
        }

        let mut descriptors: Vec<RoomDescriptor> = Vec::new();
        let num_descriptors = rng.gen_range(self.num_descriptors.clone());
        let descriptor_range = 0..=num_descriptors;
        if !descriptor_range.is_empty() {
            let mut possible_descriptors = self.possible_descriptors.clone();
            for _ in descriptor_range {
                if possible_descriptors.is_empty() {
                    break;
                }
                let index = rng.gen_range(0..possible_descriptors.len());
                let descriptor = possible_descriptors.remove(index);
                descriptors.push(descriptor);
            }
        }

        let dimensions = self.dimensions_generator.generate();

        Room {
            dimensions,
            descriptors,
            non_players,
            room_type: self.room_type.clone(),
        }
    }
}

impl RoomPrototype {
    pub fn build_random_type(npc_identifiers: Vec<Identifier>) -> RoomPrototype {
        let npc_generator = if npc_identifiers.is_empty() {
            let character_prototype = CharacterPrototype::random_species_character(None);
            NonPlayerPrototype {
                character_generators: vec![Box::new(character_prototype)],
            }
        } else {
            let mut character_generators: Vec<Box<dyn Generator<Character>>> = Vec::new();
            for identifier in npc_identifiers.iter() {
                character_generators.push(Box::new(CharacterPrototype::random_species_character(
                    Some(identifier.clone()),
                )))
            }
            NonPlayerPrototype {
                character_generators,
            }
        };

        let room_types = vec![
            RoomType::Cave,
            RoomType::Cavern,
            RoomType::EntryWay,
            RoomType::PrisonCell,
            RoomType::Room,
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..room_types.len());
        let room_type = room_types.get(index).unwrap().clone();

        RoomPrototype {
            room_type: room_type.clone(),
            non_player_generator: Box::new(npc_generator),
            num_non_players: 1..3,
            num_descriptors: 1..3,
            dimensions_generator: Box::new(DimensionsPrototype::for_room_type(&room_type)),
            possible_descriptors: room_type.possible_descriptors().clone(),
        }
    }
}
