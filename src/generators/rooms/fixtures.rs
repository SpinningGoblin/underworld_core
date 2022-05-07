use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::{
    components::{
        fixtures::{fixture::Fixture, fixture_type::FixtureType},
        rooms::{
            fixture_position::FixturePosition,
            fixture_position_descriptor::FixturePositionDescriptor,
            group_descriptor::GroupDescriptor, room_type::RoomType,
        },
    },
    generators::{fixtures::get_generator, generator::Generator},
};

pub fn build_fixture_positions(room_type: &RoomType) -> (Vec<FixturePosition>, Vec<FixtureType>) {
    let num_groups_range = 0..num_groups(room_type);
    if num_groups_range.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut rng = rand::thread_rng();

    let mut used_descriptors: Vec<FixturePositionDescriptor> = Vec::new();
    let mut used_fixtures: Vec<FixtureType> = Vec::new();
    let mut positions: Vec<FixturePosition> = Vec::new();
    for _ in num_groups_range {
        let mut fixture_generators =
            FixtureGenerators::build_with_previous(room_type, &used_fixtures);
        let fixtures: Vec<Fixture> = (0..group_size(room_type))
            .filter_map(|_| {
                if let Some(generator) = fixture_generators.next() {
                    let fixture = generator.generate();
                    if !used_fixtures.contains(&fixture.fixture_type) {
                        used_fixtures.push(fixture.fixture_type.clone());
                    }
                    Some(fixture)
                } else {
                    None
                }
            })
            .collect();

        let current_fixtures: Vec<FixtureType> =
            fixtures.iter().map(|f| f.fixture_type.clone()).collect();
        let counts = crate::utils::frequencies::sorted_frequencies(
            fixtures.iter().map(|f| f.fixture_type.clone()),
        );
        let possible_groups = if counts.get(0).unwrap().1 > 1 {
            multi_group_descriptors()
        } else {
            single_group_descriptors()
        };

        // Base the group descriptor off of how many fixtures are in the start.
        let index = rng.gen_range(0..possible_groups.len());
        let group_descriptor = possible_groups.get(index);
        let possible_positions = possible_positions(&current_fixtures);

        let mut num_position_descriptors: usize = rng.gen_range(1..=2);
        let mut position_descriptors: Vec<FixturePositionDescriptor> = Vec::new();
        while num_position_descriptors > 0 {
            let current_possibilities: Vec<FixturePositionDescriptor> = current_possibilities(
                &possible_positions,
                &used_descriptors,
                &position_descriptors,
            );

            if current_possibilities.is_empty() {
                break;
            }

            let index = rng.gen_range(0..current_possibilities.len());
            let position_descriptor = current_possibilities.get(index).unwrap();
            position_descriptors.push(position_descriptor.clone());
            used_descriptors.push(position_descriptor.clone());

            num_position_descriptors -= 1;
        }

        positions.push(FixturePosition {
            group_descriptor: group_descriptor.cloned(),
            fixtures,
            position_descriptors,
        });
    }

    (positions, used_fixtures)
}

fn current_possibilities(
    possible_positions: &[FixturePositionDescriptor],
    used_descriptors: &[FixturePositionDescriptor],
    position_descriptors: &[FixturePositionDescriptor],
) -> Vec<FixturePositionDescriptor> {
    possible_positions
        .iter()
        .filter(|descriptor| !used_descriptors.contains(*descriptor))
        .filter(|descriptor| {
            if position_descriptors.is_empty() {
                true
            } else {
                position_descriptors
                    .iter()
                    .all(|p| !p.unable_to_be_used_with(descriptor))
            }
        })
        .cloned()
        .collect()
}

fn possible_positions(fixture_types: &[FixtureType]) -> Vec<FixturePositionDescriptor> {
    let mut possibilities = if fixture_types.len() == 1 {
        single_possible_positions()
    } else {
        multi_possible_positions()
    };

    let can_be_broken_on_ground = vec![
        FixtureType::StatueWarrior,
        FixtureType::StatueTentacledMonstrosity,
        FixtureType::Pillar,
    ];

    if fixture_types
        .iter()
        .all(|fixture_type| can_be_broken_on_ground.contains(fixture_type))
    {
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

fn multi_possible_positions() -> Vec<FixturePositionDescriptor> {
    vec![
        FixturePositionDescriptor::AreInTheCorner,
        FixturePositionDescriptor::AreAlongOneSide,
        FixturePositionDescriptor::AreScatteredAboutTheRoom,
    ]
}

fn single_group_descriptors() -> Vec<GroupDescriptor> {
    vec![
        GroupDescriptor::A,
        GroupDescriptor::ALone,
        GroupDescriptor::ASingle,
    ]
}

fn multi_group_descriptors() -> Vec<GroupDescriptor> {
    vec![GroupDescriptor::Some, GroupDescriptor::AFew]
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
}

impl FixtureGenerators {
    fn build_with_previous(room_type: &RoomType, previous: &[FixtureType]) -> Self {
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
        }
    }

    fn next(&mut self) -> Option<impl Generator<Fixture>> {
        if !self.generated_once {
            self.generated_once = true;
            let fixture_type = self.fixture_types.get(self.current_index).unwrap();
            return Some(get_generator(
                fixture_type,
                has_hidden_compartment(fixture_type),
            ));
        }

        let mut rng = rand::thread_rng();
        let roll: usize = rng.gen_range(0..=100);
        let last_generated = self.fixture_types.get(self.current_index).unwrap();
        if last_generated == &FixtureType::Table && roll <= 75 {
            return Some(get_generator(
                &FixtureType::Chair,
                has_hidden_compartment(&FixtureType::Chair),
            ));
        } else if last_generated == &FixtureType::Barrel && roll <= 75 {
            return Some(get_generator(
                &FixtureType::Crate,
                has_hidden_compartment(&FixtureType::Crate),
            ));
        }

        // I don't really want to switch up generators all that often
        if roll <= 95 {
            return Some(get_generator(
                last_generated,
                has_hidden_compartment(last_generated),
            ));
        }

        let index = rng.gen_range(0..self.fixture_types.len());
        self.current_index = index;
        let fixture_type = self.fixture_types.get(index).unwrap();
        Some(get_generator(
            fixture_type,
            has_hidden_compartment(fixture_type),
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
    let roll = rng.gen_range(1..=100);
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
        _ => FixtureType::into_enum_iter().collect(),
    }
}
