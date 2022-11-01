use rand::Rng;

use crate::{
    components::{damage::AttackEffect, Attack, Defense, Health, Size, Species, Stats},
    utils::rolls::{roll_d6, roll_percent_succeeds},
};

use super::generator::Generator;

pub fn build_specific_health(
    max_health: i32,
    species: &Species,
    use_species_base: bool,
) -> StatsPrototype {
    StatsPrototype {
        max_health: Some(max_health),
        num_health_rolls: 0,
        danger_level: 1,
        species: *species,
        use_species_base,
    }
}

pub fn build_default_health_rolls(species: &Species, use_species_base: bool) -> StatsPrototype {
    let num_health_rolls = match *species {
        Species::Ogre => 5,
        Species::Dragonkin | Species::Phantom | Species::Rockoblin | Species::Shadow => 4,
        _ => 3,
    };

    StatsPrototype {
        max_health: None,
        num_health_rolls,
        danger_level: 1,
        species: *species,
        use_species_base,
    }
}

pub fn build_default_health_rolls_for_danger_level(
    species: &Species,
    danger_level: u32,
    use_species_base: bool,
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
        species: *species,
        use_species_base,
    }
}

pub struct StatsPrototype {
    pub max_health: Option<i32>,
    pub num_health_rolls: usize,
    pub danger_level: u32,
    pub species: Species,
    pub use_species_base: bool,
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

        let height = if roll_percent_succeeds(&mut rng, NON_AVERAGE_HEIGHT_CHANCE) {
            let possibilities = non_average_heights();
            let index = rng.gen_range(0..possibilities.len());
            match possibilities.get(index) {
                Some(height) => *height,
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
                    6
                } else if (21..=30).contains(&self.danger_level) {
                    8
                } else {
                    12
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

        Stats {
            health,
            height,
            base_attack: self.base_attack(),
            base_damage_resistance: self.base_damage_resistance(),
        }
    }
}

impl StatsPrototype {
    fn base_attack(&self) -> Option<Attack> {
        if !self.use_species_base {
            return None;
        }

        match self.species {
            Species::Bugbear => Some(Attack {
                num_rolls: 1,
                modifier: 0,
                effects: Vec::new(),
            }),
            Species::Dragonkin | Species::Lizardkin | Species::Orc => Some(Attack {
                num_rolls: 1,
                modifier: 1,
                effects: Vec::new(),
            }),
            Species::Goblin => Some(Attack {
                num_rolls: 0,
                modifier: 2,
                effects: Vec::new(),
            }),
            Species::Ogre | Species::Phantom | Species::Shadow => Some(Attack {
                num_rolls: 2,
                modifier: -1,
                effects: vec![AttackEffect::Crushing],
            }),
            Species::Kobold
            | Species::Moblin
            | Species::Hobgoblin
            | Species::Frogkin
            | Species::Rockoblin
            | Species::Turtlekin => None,
        }
    }

    fn base_damage_resistance(&self) -> Option<Defense> {
        if !self.use_species_base {
            return None;
        }

        match self.species {
            Species::Turtlekin => Some(Defense {
                damage_resistance: 2,
            }),
            Species::Rockoblin => Some(Defense {
                damage_resistance: 3,
            }),
            Species::Bugbear
            | Species::Dragonkin
            | Species::Frogkin
            | Species::Goblin
            | Species::Hobgoblin
            | Species::Kobold
            | Species::Lizardkin
            | Species::Moblin
            | Species::Ogre
            | Species::Orc
            | Species::Phantom
            | Species::Shadow => None,
        }
    }
}
