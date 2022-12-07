mod builder;
mod dimensions;
mod exits;
mod fixtures;
pub mod npcs;

pub use builder::{
    ExitGenerationArgs, RoomFixtureGenerationArgs, RoomGeneratorBuilder, RoomNpcGenerationArgs,
};

use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::components::{
    fixtures::FixtureType,
    rooms::{Descriptor, Dimensions, ExitType, Flavour, Room, RoomType},
    Ghost, LifeModifier, Species,
};

use self::{
    dimensions::build_dimensions, exits::build_exits, fixtures::build_fixture_positions,
    npcs::build_npc_positions,
};

use super::generator::Generator;

pub struct BuildNpcsArgs {
    pub num_groups: RangeInclusive<u16>,
    pub possible_species: Vec<Species>,
    pub possible_life_modifiers: Vec<LifeModifier>,
    pub allow_npcs_to_spawn_dead: bool,
    pub ghosts: Vec<Ghost>,
}

impl Default for BuildNpcsArgs {
    fn default() -> Self {
        Self {
            num_groups: 1..=2,
            possible_species: Species::iter().collect(),
            possible_life_modifiers: LifeModifier::iter().collect(),
            allow_npcs_to_spawn_dead: true,
            ghosts: Vec::new(),
        }
    }
}

pub struct BuildFixturesArgs {
    pub num_groups: RangeInclusive<u16>,
    pub possible_types: Vec<FixtureType>,
}

impl Default for BuildFixturesArgs {
    fn default() -> Self {
        Self {
            num_groups: 1..=2,
            possible_types: FixtureType::iter().collect(),
        }
    }
}

pub struct BuildExitArgs {
    pub num_exits: RangeInclusive<u16>,
    pub exit_types: Vec<ExitType>,
}

impl Default for BuildExitArgs {
    fn default() -> Self {
        Self {
            num_exits: 2..=3,
            exit_types: ExitType::iter().collect(),
        }
    }
}

struct RoomPrototype {
    pub num_descriptors: RangeInclusive<u16>,
    pub room_type: RoomType,
    pub possible_descriptors: Vec<Descriptor>,
    pub entrance_id: Option<Uuid>,
    pub danger_level: u32,
    pub possible_flavour_texts: Vec<Flavour>,
    pub include_flavour_text: bool,
    pub name: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub build_exit_args: BuildExitArgs,
    pub build_npc_args: BuildNpcsArgs,
    pub build_fixtures_args: BuildFixturesArgs,
}

impl Generator<Room> for RoomPrototype {
    fn generate(&self) -> Room {
        let mut rng = rand::thread_rng();

        let mut descriptors: Vec<Descriptor> = Vec::new();
        let num_descriptors = rng.gen_range(self.num_descriptors.clone());
        let descriptor_range = 0..num_descriptors;
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

        let (fixture_positions, used_fixtures) = build_fixture_positions(
            &self.build_fixtures_args,
            &self.room_type,
            self.danger_level,
        );

        Room {
            dimensions: self.dimensions.clone().unwrap_or_else(build_dimensions),
            descriptors,
            id: Uuid::new_v4(),
            name: self.name.clone(),
            room_type: self.room_type,
            fixture_positions,
            npc_positions: build_npc_positions(
                used_fixtures,
                self.danger_level,
                &self.build_npc_args,
            ),
            flavour,
            exits: build_exits(self.entrance_id, &self.build_exit_args),
            loose_items: Vec::new(),
        }
    }
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
