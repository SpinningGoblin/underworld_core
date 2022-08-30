mod dimensions;
mod exits;
mod fixtures;
pub mod npcs;

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
}

#[derive(Default)]
pub struct RoomGeneratorBuilder {
    num_descriptors: Option<RangeInclusive<usize>>,
    room_type: Option<RoomType>,
    possible_descriptors: Option<Vec<Descriptor>>,
    entrance_id: Option<Uuid>,
    danger_level: Option<u32>,
    include_flavour_text: Option<bool>,
    possible_flavour_texts: Option<Vec<Flavour>>,
    name: Option<String>,
    dimensions: Option<Dimensions>,
}

impl RoomGeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_descriptors(&mut self, num_descriptors: RangeInclusive<usize>) -> &mut Self {
        self.num_descriptors = Some(num_descriptors);

        self
    }

    pub fn room_type(&mut self, room_type: RoomType) -> &mut Self {
        self.room_type = Some(room_type);

        self
    }

    pub fn possible_descriptors(&mut self, possible_descriptors: Vec<Descriptor>) -> &mut Self {
        self.possible_descriptors = Some(possible_descriptors);

        self
    }

    pub fn entrance_id(&mut self, entrance_id: Uuid) -> &mut Self {
        self.entrance_id = Some(entrance_id);

        self
    }

    pub fn danger_level(&mut self, danger_level: u32) -> &mut Self {
        self.danger_level = Some(danger_level);

        self
    }

    pub fn include_flavour_text(&mut self, include_flavour: bool) -> &mut Self {
        self.include_flavour_text = Some(include_flavour);

        self
    }

    pub fn possible_flavour_texts(&mut self, possible_flavour_texts: Vec<Flavour>) -> &mut Self {
        self.possible_flavour_texts = Some(possible_flavour_texts);

        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());

        self
    }

    pub fn dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.dimensions = Some(dimensions);

        self
    }

    pub fn build(&self) -> impl Generator<Room> {
        let num_descriptors = match &self.num_descriptors {
            Some(it) => it.clone(),
            None => 0..=2,
        };

        let room_type = match &self.room_type {
            Some(it) => it.clone(),
            None => {
                let room_types: Vec<RoomType> = RoomType::iter().collect();
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..room_types.len());
                room_types.get(index).unwrap().clone()
            }
        };

        let possible_descriptors = match &self.possible_descriptors {
            Some(it) => it.clone(),
            None => room_type.possible_descriptors(),
        };

        let possible_flavour_texts = match &self.possible_flavour_texts {
            Some(it) => it.clone(),
            None => room_type.possible_flavours(),
        };

        RoomPrototype {
            num_descriptors,
            room_type,
            possible_descriptors,
            entrance_id: self.entrance_id,
            danger_level: self.danger_level.unwrap_or(1),
            possible_flavour_texts,
            include_flavour_text: self.include_flavour_text.unwrap_or(true),
            name: self.name.clone(),
            dimensions: self.dimensions.clone(),
        }
    }
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
        possible_flavour_texts: room_type.possible_flavours(),
        include_flavour_text: true,
        name: None,
        dimensions: None,
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
