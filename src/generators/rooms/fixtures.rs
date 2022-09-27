use rand::Rng;

use crate::{
    components::{
        fixtures::{Fixture, FixtureType},
        rooms::{FixturePosition, FixturePositionDescriptor, RoomType},
    },
    generators::{fixtures::get_generator_for_level, generator::Generator},
    utils::rolls::roll_percent_succeeds,
};

use super::BuildFixturesArgs;

pub fn build_fixture_positions(
    build_fixtures_args: &BuildFixturesArgs,
    room_type: &RoomType,
    danger_level: u32,
) -> (Vec<FixturePosition>, Vec<FixtureType>) {
    if build_fixtures_args.num_groups.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut rng = rand::thread_rng();
    let mut used_fixtures: Vec<FixtureType> = Vec::new();
    let mut positions: Vec<FixturePosition> = Vec::new();
    let num_groups = rng.gen_range(build_fixtures_args.num_groups.clone());
    for _ in 0..num_groups {
        let mut fixture_generators =
            FixtureGenerators::build(danger_level, build_fixtures_args.possible_types.clone());

        let range = 0..group_size(room_type);

        if range.is_empty() {
            continue;
        }

        for _ in range {
            let fixture = if let Some(generator) = fixture_generators.next() {
                let fixture = generator.generate();
                if !used_fixtures.contains(&fixture.fixture_type) {
                    used_fixtures.push(fixture.fixture_type);
                }
                fixture
            } else {
                continue;
            };

            let possible_positions = possible_positions(&fixture.fixture_type, room_type);
            let position_descriptor = if possible_positions.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_positions.len());
                possible_positions.get(index).cloned()
            };

            positions.push(FixturePosition {
                fixture,
                position_descriptor,
            });
        }
    }

    (positions, used_fixtures)
}

fn possible_positions(
    fixture_type: &FixtureType,
    room_type: &RoomType,
) -> Vec<FixturePositionDescriptor> {
    let mut possibilities = single_possible_positions(room_type);

    let can_be_broken_on_ground = vec![
        FixtureType::StatueWarrior,
        FixtureType::StatueTentacledMonstrosity,
        FixtureType::Pillar,
    ];

    if can_be_broken_on_ground.contains(fixture_type) {
        possibilities.push(FixturePositionDescriptor::CrackedAndBrokenOnTheGround);
    }

    possibilities
}

fn single_possible_positions(room_type: &RoomType) -> Vec<FixturePositionDescriptor> {
    match *room_type {
        RoomType::Cave
        | RoomType::Crypt
        | RoomType::EntryWay
        | RoomType::Mausoleum
        | RoomType::PrisonCell
        | RoomType::Room
        | RoomType::TavernHall
        | RoomType::TempleHall => vec![
            FixturePositionDescriptor::IsInTheCorner,
            FixturePositionDescriptor::SitsAlongOneSide,
            FixturePositionDescriptor::StandsInTheCorner,
        ],
        RoomType::Cavern | RoomType::Cemetery => Vec::new(),
    }
}

fn group_size(room_type: &RoomType) -> usize {
    let range = match *room_type {
        RoomType::PrisonCell => 1..=2,
        RoomType::EntryWay => 1..=2,
        _ => 1..=3,
    };
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

struct FixtureGenerators {
    fixture_types: Vec<FixtureType>,
    current_index: usize,
    generated_once: bool,
    danger_level: u32,
}

impl FixtureGenerators {
    fn build(danger_level: u32, fixture_types: Vec<FixtureType>) -> Self {
        let index = if fixture_types.is_empty() {
            0
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..fixture_types.len())
        };
        Self {
            fixture_types,
            danger_level,
            current_index: index,
            generated_once: false,
        }
    }

    fn next(&mut self) -> Option<impl Generator<Fixture>> {
        if self.fixture_types.is_empty() {
            return None;
        }

        if !self.generated_once {
            self.generated_once = true;
            let fixture_type = self.fixture_types.get(self.current_index).unwrap();
            return Some(get_generator_for_level(
                fixture_type,
                has_hidden_compartment(fixture_type),
                self.danger_level,
            ));
        }

        let mut rng = rand::thread_rng();
        let last_generated = self.fixture_types.get(self.current_index).unwrap();
        if last_generated == &FixtureType::Table && roll_percent_succeeds(&mut rng, 75) {
            return Some(get_generator_for_level(
                &FixtureType::Chair,
                has_hidden_compartment(&FixtureType::Chair),
                self.danger_level,
            ));
        } else if last_generated == &FixtureType::Barrel && roll_percent_succeeds(&mut rng, 75) {
            return Some(get_generator_for_level(
                &FixtureType::Crate,
                has_hidden_compartment(&FixtureType::Crate),
                self.danger_level,
            ));
        }

        // I don't really want to switch up generators all that often
        if roll_percent_succeeds(&mut rng, 95) {
            return Some(get_generator_for_level(
                last_generated,
                has_hidden_compartment(last_generated),
                self.danger_level,
            ));
        }

        let index = rng.gen_range(0..self.fixture_types.len());
        self.current_index = index;
        let fixture_type = self.fixture_types.get(index).unwrap();
        Some(get_generator_for_level(
            fixture_type,
            has_hidden_compartment(fixture_type),
            self.danger_level,
        ))
    }
}

fn has_hidden_compartment(fixture_type: &FixtureType) -> bool {
    let chance_of_hidden_compartment = match *fixture_type {
        FixtureType::Barrel => 25,
        FixtureType::Bucket | FixtureType::SleepingRoll => 0,
        FixtureType::Bed
        | FixtureType::Chair
        | FixtureType::Cot
        | FixtureType::Crate
        | FixtureType::Pillar
        | FixtureType::Table
        | FixtureType::WeaponRack => 15,
        FixtureType::Chest => 50,
        FixtureType::Coffin
        | FixtureType::StatueTentacledMonstrosity
        | FixtureType::StatueWarrior => 75,
    };

    let mut rng = rand::thread_rng();
    roll_percent_succeeds(&mut rng, chance_of_hidden_compartment)
}
