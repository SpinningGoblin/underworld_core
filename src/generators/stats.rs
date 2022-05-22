use rand::Rng;

use crate::{
    components::{damage::Health, size::Size, species::Species, stats::Stats},
    utils::rolls::{roll_d100, roll_d6},
};

use super::generator::Generator;

pub fn build_specific_health(max_health: i32) -> StatsPrototype {
    StatsPrototype {
        has_health: true,
        max_health: Some(max_health),
        num_health_rolls: 0,
    }
}

pub fn build(num_health_rolls: usize) -> StatsPrototype {
    StatsPrototype {
        has_health: true,
        max_health: None,
        num_health_rolls,
    }
}

pub fn build_default_health_rolls(species: &Species) -> StatsPrototype {
    let num_health_rolls = match *species {
        Species::Ogre => 5,
        Species::Dragonkin | Species::Phantom | Species::Rockoblin | Species::Shadow => 4,
        _ => 3,
    };

    StatsPrototype {
        has_health: true,
        max_health: None,
        num_health_rolls,
    }
}

pub struct StatsPrototype {
    pub has_health: bool,
    pub max_health: Option<i32>,
    pub num_health_rolls: usize,
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

        let health = if self.has_health {
            let max_health = match self.max_health {
                Some(max) => max,
                _ => {
                    let range = 0..self.num_health_rolls;

                    if range.is_empty() {
                        0
                    } else {
                        let mut rng = rand::thread_rng();
                        range.map(|_| roll_d6(&mut rng, 1, 0)).sum()
                    }
                }
            };
            Some(Health {
                current: max_health,
                max: max_health,
            })
        } else {
            None
        };

        Stats { health, height }
    }
}
