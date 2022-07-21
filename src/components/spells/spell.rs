#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{Attack, Defense};

use super::{SpellName, SpellType};

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
    pub uses: i32,
}

impl Spell {
    pub fn damage(&self) -> i32 {
        let mut rng = rand::thread_rng();

        match &self.attack {
            Some(attack) => attack.attack_roll(&mut rng),
            None => 0,
        }
    }

    pub fn spell_type(&self) -> SpellType {
        self.name.spell_type()
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
    pub uses: i32,
    pub knows_uses: bool,
    pub spell_type: SpellType,
}
