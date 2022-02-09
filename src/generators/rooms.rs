mod dimensions;
mod fixtures;
mod npcs;

use std::ops::Range;

use enum_iterator::IntoEnumIterator;
use rand::Rng;
use uuid::Uuid;

use crate::components::{
    identifier::Identifier,
    non_player::NonPlayer,
    rooms::{descriptor::Descriptor, room::Room, room_type::RoomType},
    species::Species,
};

use self::{dimensions::build_dimensions, fixtures::build_fixture_positions};

use super::{
    characters::CharacterPrototype, generator::Generator, non_players::NonPlayerPrototype,
};

pub struct RoomPrototype {
    pub possible_npc_names: Vec<String>,
    pub non_player_generators: Vec<Box<dyn Generator<NonPlayer>>>,
    pub num_non_players: Range<usize>,
    pub num_descriptors: Range<usize>,
    pub room_type: RoomType,
    pub possible_descriptors: Vec<Descriptor>,
}

const SWITCH_GENERATOR_CHANCE: usize = 25;

impl RoomPrototype {
    fn switch_npc_generator() -> bool {
        let mut rng = rand::thread_rng();
        let switch_gen_roll: usize = rng.gen_range(0..=100);
        switch_gen_roll <= SWITCH_GENERATOR_CHANCE
    }
}

impl Generator<Room> for RoomPrototype {
    fn generate(&self) -> Room {
        let mut rng = rand::thread_rng();
        let num_non_players = rng.gen_range(self.num_non_players.clone());

        let mut non_players: Vec<NonPlayer> = Vec::new();
        let non_player_range = 0..=num_non_players;
        let mut names = self.possible_npc_names.clone();

        if !non_player_range.is_empty() {
            let npc_generator_index = rng.gen_range(0..self.non_player_generators.len());
            let mut npc_generator = self.non_player_generators.get(npc_generator_index).unwrap();
            for _ in 1..=num_non_players {
                if Self::switch_npc_generator() {
                    let generator_index = rng.gen_range(0..self.non_player_generators.len());
                    npc_generator = self.non_player_generators.get(generator_index).unwrap();
                }

                let mut non_player = npc_generator.generate();
                if !names.is_empty() {
                    let name_index = rng.gen_range(0..names.len());
                    let name = names.remove(name_index);
                    non_player.set_name(&name);
                }
                non_players.push(non_player);
            }
        }

        let mut descriptors: Vec<Descriptor> = Vec::new();
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

        Room {
            dimensions: build_dimensions(),
            descriptors,
            non_players,
            identifier: Identifier {
                id: Uuid::new_v4(),
                name: None,
            },
            room_type: self.room_type.clone(),
            fixture_positions: build_fixture_positions(&self.room_type),
            npc_positions: Vec::new(),
        }
    }
}

impl RoomPrototype {
    pub fn build_random(npc_names: Vec<String>, num_non_players: Range<usize>) -> RoomPrototype {
        let mut npc_generators: Vec<Box<dyn Generator<NonPlayer>>> = Vec::new();

        for species in Species::into_enum_iter() {
            npc_generators.push(Box::new(NonPlayerPrototype {
                character_generator: Box::new(CharacterPrototype::overloaded_character(species)),
                name: None,
            }));
        }

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
            num_non_players,
            possible_npc_names: npc_names,
            room_type: room_type.clone(),
            non_player_generators: npc_generators,
            num_descriptors: 1..3,
            possible_descriptors: room_type.possible_descriptors(),
        }
    }
}

impl RoomType {
    fn possible_descriptors(&self) -> Vec<Descriptor> {
        match *self {
            RoomType::Cave => Descriptor::into_enum_iter().collect(),
            RoomType::Cavern => Descriptor::into_enum_iter().collect(),
            RoomType::PrisonCell => Descriptor::into_enum_iter().collect(),
            RoomType::Room => Descriptor::into_enum_iter().collect(),
            RoomType::EntryWay => Descriptor::into_enum_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generators::generator::Generator;

    use super::RoomPrototype;

    #[test]
    fn generate_room() {
        let room_prototype = RoomPrototype::build_random(Vec::new(), 1..3);
        let room = room_prototype.generate();
        assert!(!format!("{}", &room).is_empty());
    }
}
