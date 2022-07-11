use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    components::{
        fixtures::{Fixture, FixtureType},
        rooms::{FixturePosition, FixturePositionDescriptor, GroupDescriptor, RoomType},
    },
    generators::{fixtures::get_generator_for_level, generator::Generator},
    utils::rolls::roll_d100,
};

pub fn build_fixture_positions(
    room_type: &RoomType,
    danger_level: u32,
) -> (Vec<FixturePosition>, Vec<FixtureType>) {
    let num_groups_range = 0..num_groups(room_type);
    if num_groups_range.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut rng = rand::thread_rng();
    let mut used_fixtures: Vec<FixtureType> = Vec::new();
    let mut positions: Vec<FixturePosition> = Vec::new();
    for _ in num_groups_range {
        let mut fixture_generators =
            FixtureGenerators::build_with_previous(room_type, &used_fixtures, danger_level);

        let range = 0..group_size(room_type);

        if range.is_empty() {
            continue;
        }

        for _ in range {
            let fixture = if let Some(generator) = fixture_generators.next() {
                let fixture = generator.generate();
                if !used_fixtures.contains(&fixture.fixture_type) {
                    used_fixtures.push(fixture.fixture_type.clone());
                }
                fixture
            } else {
                continue;
            };

            let group_descriptors = single_group_descriptors();
            let group_descriptor = if group_descriptors.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..group_descriptors.len());
                group_descriptors.get(index)
            };

            let possible_positions = possible_positions(&fixture.fixture_type);
            let position_descriptor = if possible_positions.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_positions.len());
                possible_positions.get(index).cloned()
            };

            positions.push(FixturePosition {
                group_descriptor: group_descriptor.cloned(),
                fixture,
                position_descriptor,
            });
        }
    }

    (positions, used_fixtures)
}

fn possible_positions(fixture_type: &FixtureType) -> Vec<FixturePositionDescriptor> {
    let mut possibilities = single_possible_positions();

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

fn single_possible_positions() -> Vec<FixturePositionDescriptor> {
    vec![
        FixturePositionDescriptor::IsInTheCorner,
        FixturePositionDescriptor::SitsAlongOneSide,
        FixturePositionDescriptor::StandsInTheCorner,
    ]
}

fn single_group_descriptors() -> Vec<GroupDescriptor> {
    vec![
        GroupDescriptor::A,
        GroupDescriptor::ALone,
        GroupDescriptor::ASingle,
    ]
}

fn num_groups(room_type: &RoomType) -> usize {
    let range = match *room_type {
        RoomType::PrisonCell => 0..=1,
        RoomType::Room => 0..=1,
        RoomType::EntryWay => 0..=1,
        _ => 0..=2,
    };

    let mut rng = rand::thread_rng();
    rng.gen_range(range)
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
    fn build_with_previous(
        room_type: &RoomType,
        previous: &[FixtureType],
        danger_level: u32,
    ) -> Self {
        let possible_fixtures: Vec<FixtureType> = possible_fixtures(room_type)
            .iter()
            .filter(|fixture| !previous.contains(fixture))
            .cloned()
            .collect();
        let index = if possible_fixtures.is_empty() {
            0
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..possible_fixtures.len())
        };
        Self {
            fixture_types: possible_fixtures,
            current_index: index,
            generated_once: false,
            danger_level,
        }
    }

    fn next(&mut self) -> Option<impl Generator<Fixture>> {
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
        let roll = roll_d100(&mut rng, 1, 0);
        let last_generated = self.fixture_types.get(self.current_index).unwrap();
        if last_generated == &FixtureType::Table && roll <= 75 {
            return Some(get_generator_for_level(
                &FixtureType::Chair,
                has_hidden_compartment(&FixtureType::Chair),
                self.danger_level,
            ));
        } else if last_generated == &FixtureType::Barrel && roll <= 75 {
            return Some(get_generator_for_level(
                &FixtureType::Crate,
                has_hidden_compartment(&FixtureType::Crate),
                self.danger_level,
            ));
        }

        // I don't really want to switch up generators all that often
        if roll <= 95 {
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
    let roll = roll_d100(&mut rng, 1, 0);
    roll <= chance_of_hidden_compartment
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
