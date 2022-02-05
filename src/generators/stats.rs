use std::ops::Range;

use rand::Rng;

use crate::components::{health::Health, size::Size, species::Species, stats::Stats};

use super::generator::Generator;

pub struct StatsPrototype {
    pub height_range: Range<f32>,
    pub width_range: Range<f32>,
    pub health_range: Range<i32>,
    pub has_health: bool,
    pub has_dimensions: bool,
    pub max_health: Option<i32>,
}

impl StatsPrototype {
    pub fn bugbear(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 1.0..2.2,
            width_range: 0.5..0.75,
            health_range: 10..16,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn goblin(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 0.4..1.4,
            width_range: 0.5..0.75,
            health_range: 8..13,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn hobgoblin(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 0.4..1.4,
            width_range: 0.5..0.75,
            health_range: 8..13,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn kobold(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 0.4..1.4,
            width_range: 0.5..0.75,
            health_range: 8..13,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn ogre(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 1.8..4.6,
            width_range: 0.75..1.0,
            health_range: 12..20,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn orc(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 1.0..2.2,
            width_range: 0.5..0.75,
            health_range: 10..16,
            has_health: true,
            has_dimensions: true,
        }
    }

    pub fn unknown(max_health: Option<i32>) -> Self {
        Self {
            max_health,
            height_range: 0.4..4.6,
            width_range: 0.5..0.75,
            health_range: 3..20,
            has_health: true,
            has_dimensions: true,
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

impl Generator<Stats> for StatsPrototype {
    fn generate(&self) -> Stats {
        let mut rng = rand::thread_rng();

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

        Stats {
            health,
            height: Size::Average,
        }
    }
}
