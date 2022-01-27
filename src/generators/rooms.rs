use std::ops::Range;

use rand::Rng;
use uuid::Uuid;

use crate::components::{
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
    pub non_player_generators: Vec<Box<dyn Generator<NonPlayer>>>,
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
                let npc_generator_index = rng.gen_range(0..self.non_player_generators.len());
                let npc_generator = self.non_player_generators.get(npc_generator_index).unwrap();
                let non_player = npc_generator.generate();
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
            identifier: Identifier {
                id: Uuid::new_v4(),
                name: None,
            },
            room_type: self.room_type.clone(),
        }
    }
}

impl RoomPrototype {
    pub fn build_random(npc_names: Vec<String>) -> RoomPrototype {
        let npc_generators: Vec<Box<dyn Generator<NonPlayer>>> = if npc_names.is_empty() {
            let character_prototype = CharacterPrototype::random_species_character();
            vec![Box::new(NonPlayerPrototype {
                name: None,
                character_generator: Box::new(character_prototype),
            })]
        } else {
            let mut generators: Vec<Box<dyn Generator<NonPlayer>>> = Vec::new();
            for name in npc_names.iter() {
                generators.push(Box::new(NonPlayerPrototype {
                    name: Some(name.clone()),
                    character_generator: Box::new(CharacterPrototype::random_species_character()),
                }));
            }

            generators
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
            non_player_generators: npc_generators,
            num_non_players: 1..3,
            num_descriptors: 1..3,
            dimensions_generator: Box::new(DimensionsPrototype::for_room_type(&room_type)),
            possible_descriptors: room_type.possible_descriptors().clone(),
        }
    }
}
