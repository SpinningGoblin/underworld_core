#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use rand::{prelude::ThreadRng, Rng};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Attack {
    pub num_rolls: usize,
    pub modifier: i32,
}

impl Attack {
    pub fn attack_roll(&self, rng: &mut ThreadRng) -> i32 {
        roll(rng, self.num_rolls, self.modifier)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Defense {
    pub num_rolls: usize,
    pub modifier: i32,
}

impl Defense {
    pub fn defense_roll(&self, rng: &mut ThreadRng) -> i32 {
        roll(rng, self.num_rolls, self.modifier)
    }
}

fn roll(rng: &mut ThreadRng, num_rolls: usize, modifier: i32) -> i32 {
    let roll: i32 = (0..num_rolls)
        .map(|_| -> i32 { rng.gen_range(1..=6) })
        .sum();
    0.max(roll - modifier)
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
