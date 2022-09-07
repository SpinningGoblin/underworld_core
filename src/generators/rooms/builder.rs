use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    components::rooms::{Descriptor, Dimensions, Flavour, Room, RoomType},
    generators::generator::Generator,
};

use super::{exits::ExitGenerationArgs, RoomPrototype};

#[derive(Default)]
pub struct RoomGeneratorBuilder {
    num_descriptors: Option<RangeInclusive<u16>>,
    room_type: Option<RoomType>,
    possible_descriptors: Option<Vec<Descriptor>>,
    entrance_id: Option<Uuid>,
    danger_level: Option<u32>,
    include_flavour_text: Option<bool>,
    possible_flavour_texts: Option<Vec<Flavour>>,
    name: Option<String>,
    dimensions: Option<Dimensions>,
    exit_generation_args: Option<ExitGenerationArgs>,
}

impl RoomGeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_descriptors(&mut self, num_descriptors: RangeInclusive<u16>) -> &mut Self {
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

    pub fn exit_generation_args(&mut self, exit_generation_args: ExitGenerationArgs) -> &mut Self {
        self.exit_generation_args = Some(exit_generation_args);

        self
    }

    pub fn build(&self) -> impl Generator<Room> {
        let num_descriptors = match &self.num_descriptors {
            Some(it) => it.clone(),
            None => 0..=2,
        };

        let room_type = match &self.room_type {
            Some(it) => *it,
            None => {
                let room_types: Vec<RoomType> = RoomType::iter().collect();
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..room_types.len());
                *room_types.get(index).unwrap()
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

        let exit_generation_args = match &self.exit_generation_args {
            Some(it) => it.clone(),
            None => ExitGenerationArgs::default(),
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
            exit_generation_args,
        }
    }
}
