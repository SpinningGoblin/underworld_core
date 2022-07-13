use rand::Rng;

use crate::{
    components::{size::Size, species::Species, stats::Stats, Health},
    utils::rolls::{roll_d100, roll_d6},
};

use super::generator::Generator;

pub fn build_specific_health(max_health: i32) -> StatsPrototype {
    StatsPrototype {
        max_health: Some(max_health),
        num_health_rolls: 0,
        danger_level: 1,
    }
}

pub fn build_specific_health_danger_level(max_health: i32, danger_level: u32) -> StatsPrototype {
    StatsPrototype {
        max_health: Some(max_health),
        num_health_rolls: 0,
        danger_level,
    }
}

pub fn build(num_health_rolls: usize) -> StatsPrototype {
    StatsPrototype {
        max_health: None,
        num_health_rolls,
        danger_level: 1,
    }
}

pub fn build_danger_level(num_health_rolls: usize, danger_level: u32) -> StatsPrototype {
    StatsPrototype {
        max_health: None,
        num_health_rolls,
        danger_level,
    }
}

pub fn build_default_health_rolls(species: &Species) -> StatsPrototype {
    let num_health_rolls = match *species {
        Species::Ogre => 5,
        Species::Dragonkin | Species::Phantom | Species::Rockoblin | Species::Shadow => 4,
        _ => 3,
    };

    StatsPrototype {
        max_health: None,
        num_health_rolls,
        danger_level: 1,
    }
}

pub fn build_default_health_rolls_for_danger_level(
    species: &Species,
    danger_level: u32,
) -> StatsPrototype {
    let num_health_rolls = match *species {
        Species::Ogre => 5,
        Species::Dragonkin | Species::Phantom | Species::Rockoblin | Species::Shadow => 4,
        _ => 3,
    };

    StatsPrototype {
        max_health: None,
        num_health_rolls,
        danger_level,
    }
}

pub struct StatsPrototype {
    pub max_health: Option<i32>,
    pub num_health_rolls: usize,
    pub danger_level: u32,
}

const NON_AVERAGE_HEIGHT_CHANCE: i32 = 40;

fn non_average_heights() -> Vec<Size> {
    vec![
        Size::Tall,
        Size::Short,
        Size::Squat,
        Size::Huge,
        Size::Massive,
    ]
}

impl Generator<Stats> for StatsPrototype {
    fn generate(&self) -> Stats {
        let mut rng = rand::thread_rng();

        let non_average_height_roll = roll_d100(&mut rng, 1, 0);
        let height = if non_average_height_roll <= NON_AVERAGE_HEIGHT_CHANCE {
            let possibilities = non_average_heights();
            let index = rng.gen_range(0..possibilities.len());
            match possibilities.get(index) {
                Some(height) => height.clone(),
                None => Size::Average,
            }
        } else {
            Size::Average
        };

        let max_health = match self.max_health {
            Some(max) => max,
            _ => {
                let additional_rolls = if (1..=5).contains(&self.danger_level) {
                    0
                } else if (6..=10).contains(&self.danger_level) {
                    2
                } else if (11..=15).contains(&self.danger_level) {
                    4
                } else if (16..=20).contains(&self.danger_level) {
                    5
                } else if (21..=30).contains(&self.danger_level) {
                    6
                } else {
                    8
                };

                let num_rolls = self.num_health_rolls + additional_rolls;
                let range = 0..num_rolls;

                let min_health_roll = if (1..=10).contains(&self.danger_level) {
                    1
                } else if (11..=30).contains(&self.danger_level) {
                    2
                } else {
                    3
                };

                if range.is_empty() {
                    0
                } else {
                    let mut rng = rand::thread_rng();
                    range
                        .map(|_| min_health_roll.max(roll_d6(&mut rng, 1, 0)))
                        .sum()
                }
            }
        };
        let health = Health {
            current: max_health,
            max: max_health,
        };

        Stats { health, height }
    }
}
