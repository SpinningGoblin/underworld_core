use std::ops::RangeInclusive;

use rand::Rng;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    components::{
        fixtures::FixtureType,
        rooms::{Descriptor, Dimensions, ExitType, Flavour, Room, RoomType},
        Ghost, LifeModifier, Species,
    },
    generators::generator::Generator,
};

use super::{BuildExitArgs, BuildFixturesArgs, BuildNpcsArgs, RoomPrototype};

#[derive(Default, Clone)]
pub struct ExitGenerationArgs {
    pub num_exits: Option<RangeInclusive<u16>>,
    pub possible_exit_types: Option<Vec<ExitType>>,
}

#[derive(Default, Clone)]
pub struct RoomNpcGenerationArgs {
    pub num_groups: Option<RangeInclusive<u16>>,
    pub possible_species: Option<Vec<Species>>,
    pub possible_life_modifiers: Option<Vec<LifeModifier>>,
    pub allow_npcs_to_spawn_dead: Option<bool>,
    pub ghosts: Option<Vec<Ghost>>,
}

#[derive(Default, Clone)]
pub struct RoomFixtureGenerationArgs {
    pub num_groups: Option<RangeInclusive<u16>>,
    pub possible_types: Option<Vec<FixtureType>>,
}

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
    room_npc_generation_args: Option<RoomNpcGenerationArgs>,
    room_fixture_generation_args: Option<RoomFixtureGenerationArgs>,
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

    pub fn room_fixture_generation_args(
        &mut self,
        room_fixture_generation_args: RoomFixtureGenerationArgs,
    ) -> &mut Self {
        self.room_fixture_generation_args = Some(room_fixture_generation_args);

        self
    }

    pub fn room_npc_generation_args(
        &mut self,
        room_npc_generation_args: RoomNpcGenerationArgs,
    ) -> &mut Self {
        self.room_npc_generation_args = Some(room_npc_generation_args);

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

        let build_exit_args = match &self.exit_generation_args {
            Some(exit_generation_args) => {
                let num_exits = match &exit_generation_args.num_exits {
                    Some(it) => it.clone(),
                    None => num_exits(&room_type),
                };

                let exit_types = match &exit_generation_args.possible_exit_types {
                    Some(it) => it.clone(),
                    None => exit_types(&room_type),
                };

                BuildExitArgs {
                    num_exits,
                    exit_types,
                }
            }
            None => BuildExitArgs {
                num_exits: num_exits(&room_type),
                exit_types: exit_types(&room_type),
            },
        };

        let build_npc_args = match &self.room_npc_generation_args {
            Some(room_npc_generation_args) => {
                let num_groups = match &room_npc_generation_args.num_groups {
                    Some(it) => it.clone(),
                    None => num_groups(&room_type),
                };

                let possible_species = match &room_npc_generation_args.possible_species {
                    Some(it) => it.clone(),
                    None => Species::iter().collect(),
                };

                let possible_life_modifiers =
                    match &room_npc_generation_args.possible_life_modifiers {
                        Some(it) => it.clone(),
                        None => LifeModifier::iter().collect(),
                    };

                let allow_npcs_to_spawn_dead =
                    match &room_npc_generation_args.allow_npcs_to_spawn_dead {
                        Some(it) => *it,
                        None => true,
                    };

                let ghosts = match &room_npc_generation_args.ghosts {
                    Some(it) => it.to_vec(),
                    None => Vec::new(),
                };

                BuildNpcsArgs {
                    num_groups,
                    possible_species,
                    possible_life_modifiers,
                    allow_npcs_to_spawn_dead,
                    ghosts,
                }
            }
            None => BuildNpcsArgs {
                num_groups: num_groups(&room_type),
                possible_species: Species::iter().collect(),
                possible_life_modifiers: LifeModifier::iter().collect(),
                allow_npcs_to_spawn_dead: true,
                ghosts: Vec::new(),
            },
        };

        let build_fixtures_args = match &self.room_fixture_generation_args {
            Some(room_fixture_gen_args) => {
                let num_groups = match &room_fixture_gen_args.num_groups {
                    Some(it) => it.to_owned(),
                    None => num_fixture_groups(&room_type),
                };

                let possible_types = match &room_fixture_gen_args.possible_types {
                    Some(it) => it.to_owned(),
                    None => possible_fixtures(&room_type),
                };

                BuildFixturesArgs {
                    num_groups,
                    possible_types,
                }
            }
            None => BuildFixturesArgs {
                num_groups: num_fixture_groups(&room_type),
                possible_types: possible_fixtures(&room_type),
            },
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
            build_exit_args,
            build_npc_args,
            build_fixtures_args,
        }
    }
}

fn possible_fixtures(room_type: &RoomType) -> Vec<FixtureType> {
    match *room_type {
        RoomType::PrisonCell => vec![
            FixtureType::Bucket,
            FixtureType::Cot,
            FixtureType::SleepingRoll,
        ],
        RoomType::EntryWay => vec![
            FixtureType::Barrel,
            FixtureType::Bucket,
            FixtureType::Chair,
            FixtureType::Chest,
            FixtureType::WeaponRack,
        ],
        RoomType::TavernHall => vec![
            FixtureType::Chair,
            FixtureType::Table,
            FixtureType::Barrel,
            FixtureType::Crate,
            FixtureType::Bucket,
        ],
        _ => FixtureType::iter().collect(),
    }
}

fn num_exits(room_type: &RoomType) -> RangeInclusive<u16> {
    match *room_type {
        RoomType::PrisonCell => 1..=2,
        RoomType::Cavern
        | RoomType::TavernHall
        | RoomType::Mausoleum
        | RoomType::Cemetery
        | RoomType::Crypt
        | RoomType::TempleHall
        | RoomType::Cave
        | RoomType::Room => 3..=5,
        RoomType::EntryWay => 2..=2,
    }
}

fn num_fixture_groups(room_type: &RoomType) -> RangeInclusive<u16> {
    match *room_type {
        RoomType::PrisonCell => 0..=1,
        RoomType::Room => 0..=1,
        RoomType::EntryWay => 0..=1,
        _ => 0..=2,
    }
}

fn num_groups(room_type: &RoomType) -> RangeInclusive<u16> {
    match *room_type {
        RoomType::PrisonCell | RoomType::EntryWay | RoomType::Mausoleum => 0..=1,
        RoomType::Cave | RoomType::Crypt | RoomType::Room | RoomType::TempleHall => 1..=2,
        RoomType::Cemetery | RoomType::Cavern | RoomType::TavernHall => 1..=3,
    }
}

fn exit_types(room_type: &RoomType) -> Vec<ExitType> {
    match *room_type {
        RoomType::PrisonCell => vec![
            ExitType::DugOutTunnelEntrance,
            ExitType::Door,
            ExitType::OpeningToTheVoid,
            ExitType::HoleInTheFloor,
            ExitType::HoleInTheWall,
        ],
        _ => ExitType::iter().collect(),
    }
}
