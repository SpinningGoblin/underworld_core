#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use rand::rngs::ThreadRng;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{Attack, Defense, Health, Size};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Stats {
    pub health: Health,
    pub height: Size,
    pub base_attack: Option<Attack>,
    pub base_damage_resistance: Option<Defense>,
}

impl Stats {
    pub fn base_attack_roll(&self, rng: &mut ThreadRng) -> i32 {
        self.base_attack
            .as_ref()
            .map(|attack| attack.attack_roll(rng))
            .unwrap_or_default()
    }

    pub fn base_damage_resistance(&self) -> i32 {
        self.base_damage_resistance
            .as_ref()
            .map(|defense| defense.damage_resistance)
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Stats"))]
pub struct StatsView {
    pub health: Option<Health>,
    pub health_known: bool,
    pub height: Size,
}
