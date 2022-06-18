mod dimensions;
mod exits;
mod fixtures;
pub mod npcs;

use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::components::rooms::{
    descriptor::Descriptor, flavour::Flavour, room::Room, room_type::RoomType,
};

use self::{
    dimensions::build_dimensions, exits::build_exits, fixtures::build_fixture_positions,
    npcs::build_npc_positions,
};

use super::generator::Generator;

struct RoomPrototype {
    pub num_descriptors: RangeInclusive<usize>,
    pub room_type: RoomType,
    pub possible_descriptors: Vec<Descriptor>,
    pub entrance_id: Option<Uuid>,
    pub danger_level: u32,
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

        let flavour_options = self.room_type.possible_flavours();
        let index = rng.gen_range(0..flavour_options.len());
        let flavour = flavour_options.get(index).cloned();

        let (fixture_positions, used_fixtures) = build_fixture_positions(&self.room_type);

        Room {
            dimensions: build_dimensions(),
            descriptors,
            id: Uuid::new_v4(),
            name: None,
            room_type: self.room_type.clone(),
            fixture_positions,
            npc_positions: build_npc_positions(&self.room_type, used_fixtures, self.danger_level),
            flavour,
            exits: build_exits(&self.room_type, self.entrance_id),
        }
    }
}

pub fn room_generator(room_type: &RoomType, entrance_id: Option<Uuid>) -> impl Generator<Room> {
    RoomPrototype {
        num_descriptors: 1..=2,
        room_type: room_type.clone(),
        possible_descriptors: room_type.possible_descriptors(),
        entrance_id,
        danger_level: 1,
    }
}

pub fn room_generator_for_danger_level(
    room_type: &RoomType,
    entrance_id: Option<Uuid>,
    danger_level: u32,
) -> impl Generator<Room> {
    RoomPrototype {
        num_descriptors: 1..=2,
        room_type: room_type.clone(),
        possible_descriptors: room_type.possible_descriptors(),
        entrance_id,
        danger_level,
    }
}

pub fn random_room_generator(entrance_id: Option<Uuid>) -> impl Generator<Room> {
    let room_types: Vec<RoomType> = RoomType::iter().collect();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..room_types.len());
    let room_type = room_types.get(index).unwrap();

    room_generator(room_type, entrance_id)
}

pub fn random_room_generator_for_danger_level(
    entrance_id: Option<Uuid>,
    danger_level: u32,
) -> impl Generator<Room> {
    let room_types: Vec<RoomType> = RoomType::iter().collect();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..room_types.len());
    let room_type = room_types.get(index).unwrap();

    room_generator_for_danger_level(room_type, entrance_id, danger_level)
}

impl RoomType {
    fn possible_descriptors(&self) -> Vec<Descriptor> {
        Descriptor::iter().collect()
    }

    fn possible_flavours(&self) -> Vec<Flavour> {
        Flavour::iter().collect()
    }
}
