use std::ops::Range;

use rand::Rng;

use crate::components::{health::Health, size::Size, species::Species, stats::Stats};

use super::generator::Generator;

pub struct StatsPrototype {
    pub health_range: Range<i32>,
    pub has_health: bool,
    pub max_health: Option<i32>,
}

impl StatsPrototype {
    pub fn bugbear(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 10..16,
            has_health: true,
        }
    }

    pub fn goblin(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 8..13,
            has_health: true,
        }
    }

    pub fn hobgoblin(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 8..13,
            has_health: true,
        }
    }

    pub fn kobold(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 8..13,
            has_health: true,
        }
    }

    pub fn ogre(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 12..20,
            has_health: true,
        }
    }

    pub fn orc(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 10..16,
            has_health: true,
        }
    }

    pub fn unknown(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            health_range: 3..20,
            has_health: true,
        }
    }

    pub fn specific_health(species: &Species, max_health: i32) -> Self {
        match *species {
            Species::Bugbear => Self::bugbear(Some(max_health)),
            Species::Goblin => Self::goblin(Some(max_health)),
            Species::Kobold => Self::kobold(Some(max_health)),
            Species::Ogre => Self::ogre(Some(max_health)),
            Species::Orc => Self::orc(Some(max_health)),
            Species::Unknown => Self::unknown(Some(max_health)),
            Species::Hobgoblin => Self::hobgoblin(Some(max_health)),
        }
    }
}

impl From<&Species> for StatsPrototype {
    fn from(species: &Species) -> Self {
        match *species {
            Species::Bugbear => Self::bugbear(None),
            Species::Goblin => Self::goblin(None),
            Species::Kobold => Self::kobold(None),
            Species::Ogre => Self::ogre(None),
            Species::Orc => Self::orc(None),
            Species::Unknown => Self::unknown(None),
            Species::Hobgoblin => Self::hobgoblin(None),
        }
    }
}

impl StatsPrototype {
    pub fn non_average_heights() -> Vec<Size> {
        vec![
            Size::Tall,
            Size::Short,
            Size::Squat,
            Size::Huge,
            Size::Massive,
        ]
    }
}

const NON_AVERAGE_HEIGHT_CHANCE: usize = 40;

impl Generator<Stats> for StatsPrototype {
    fn generate(&self) -> Stats {
        let mut rng = rand::thread_rng();

        let non_average_height_roll: usize = rng.gen_range(0..=100);
        let height = if non_average_height_roll <= NON_AVERAGE_HEIGHT_CHANCE {
            let possibilities = Self::non_average_heights();
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
                _ => rng.gen_range(self.health_range.clone()),
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
