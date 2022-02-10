mod dimensions;
mod fixtures;
mod npcs;

use std::ops::RangeInclusive;

use enum_iterator::IntoEnumIterator;
use rand::Rng;
use uuid::Uuid;

use crate::components::{
    identifier::Identifier,
    non_player::NonPlayer,
    rooms::{descriptor::Descriptor, room::Room, room_type::RoomType},
    species::Species,
};

use self::{
    dimensions::build_dimensions, fixtures::build_fixture_positions, npcs::build_npc_positions,
};

use super::{
    characters::CharacterPrototype, generator::Generator, non_players::NonPlayerPrototype,
};

pub struct RoomPrototype {
    pub num_descriptors: RangeInclusive<usize>,
    pub room_type: RoomType,
    pub possible_descriptors: Vec<Descriptor>,
}

impl Generator<Room> for RoomPrototype {
    fn generate(&self) -> Room {
        let mut rng = rand::thread_rng();

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

        let (fixture_positions, used_fixtures) = build_fixture_positions(&self.room_type);

        Room {
            dimensions: build_dimensions(),
            descriptors,
            identifier: Identifier {
                id: Uuid::new_v4(),
                name: None,
            },
            room_type: self.room_type.clone(),
            fixture_positions,
            npc_positions: build_npc_positions(&self.room_type, used_fixtures),
        }
    }
}

impl RoomPrototype {
    pub fn build_random() -> RoomPrototype {
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
            room_type: room_type.clone(),
            num_descriptors: 1..=2,
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
        let room_prototype = RoomPrototype::build_random();
        let room = room_prototype.generate();
        assert!(!format!("{}", &room).is_empty());
    }
}
