mod builder;
mod dimensions;
mod exits;
mod fixtures;
pub mod npcs;

pub use builder::RoomGeneratorBuilder;
pub use exits::ExitGenerationArgs;

use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::components::rooms::{Descriptor, Dimensions, Flavour, Room, RoomType};

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
    pub possible_flavour_texts: Vec<Flavour>,
    pub include_flavour_text: bool,
    pub name: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub exit_generation_args: ExitGenerationArgs,
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

        let flavour = if self.include_flavour_text {
            let index = rng.gen_range(0..self.possible_flavour_texts.len());
            self.possible_flavour_texts.get(index).cloned()
        } else {
            None
        };

        let (fixture_positions, used_fixtures) =
            build_fixture_positions(&self.room_type, self.danger_level);

        Room {
            dimensions: self.dimensions.clone().unwrap_or_else(build_dimensions),
            descriptors,
            id: Uuid::new_v4(),
            name: self.name.clone(),
            room_type: self.room_type.clone(),
            fixture_positions,
            npc_positions: build_npc_positions(&self.room_type, used_fixtures, self.danger_level),
            flavour,
            exits: build_exits(
                &self.room_type,
                self.entrance_id,
                &self.exit_generation_args,
            ),
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
        possible_flavour_texts: room_type.possible_flavours(),
        include_flavour_text: true,
        name: None,
        dimensions: None,
        exit_generation_args: ExitGenerationArgs::default(),
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
        possible_flavour_texts: room_type.possible_flavours(),
        include_flavour_text: true,
        name: None,
        dimensions: None,
        exit_generation_args: ExitGenerationArgs::default(),
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        components::rooms::{Descriptor, RoomType},
        generators::generator::Generator,
    };

    use super::RoomGeneratorBuilder;

    #[test]
    fn test_builder() {
        let entrance_id = Uuid::new_v4();
        let generator = RoomGeneratorBuilder::new()
            .num_descriptors(1..=1)
            .room_type(RoomType::TavernHall)
            .possible_descriptors(vec![Descriptor::Freezing])
            .entrance_id(entrance_id)
            .danger_level(10)
            .build();
        let room = generator.generate();

        assert_eq!(RoomType::TavernHall, room.room_type);
        assert_eq!(1, room.descriptors.len());
        assert_eq!(
            Descriptor::Freezing,
            room.descriptors.get(0).unwrap().clone()
        );
    }
}
