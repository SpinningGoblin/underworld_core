#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::damage::{Attack, Defense};

use super::spell_name::SpellName;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Spell {
    pub name: SpellName,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub attack: Option<Attack>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub defense: Option<Defense>,
    pub single_use: bool,
}

impl Spell {
    pub fn damage(&self) -> i32 {
        let mut rng = rand::thread_rng();

        match &self.attack {
            Some(attack) => attack.attack_roll(&mut rng),
            None => 0,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Spell"))]
pub struct SpellView {
    pub name: SpellName,
    pub attack: Option<Attack>,
    pub knows_attack: bool,
    pub defense: Option<Defense>,
    pub knows_defense: bool,
    pub single_use: bool,
    pub knows_single_use: bool,
}
