#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use rand::prelude::ThreadRng;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Attack {
    pub num_rolls: usize,
    pub modifier: i32,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub effects: Vec<AttackEffect>,
}

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum AttackEffect {
    Acidic,
    Crushing,
    Sharp,
    Toxic,
}

impl Attack {
    pub fn attack_roll(&self, rng: &mut ThreadRng) -> i32 {
        let roll = crate::utils::rolls::roll_d6(rng, self.num_rolls, self.modifier);
        if self
            .effects
            .iter()
            .any(|effect| matches!(*effect, AttackEffect::Crushing))
        {
            roll + (roll / 2)
        } else {
            roll
        }
    }

    pub fn attack_damage(&self, rng: &mut ThreadRng) -> AttackDamage {
        let roll = crate::utils::rolls::roll_d6(rng, self.num_rolls, self.modifier);

        let damage = if self
            .effects
            .iter()
            .any(|effect| matches!(*effect, AttackEffect::Crushing))
        {
            roll + (roll / 2)
        } else {
            roll
        };

        AttackDamage {
            damage,
            effects: self.effects.clone(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct AttackDamage {
    pub damage: i32,
    pub effects: Vec<AttackEffect>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Defense {
    pub damage_resistance: i32,
}

impl Defense {
    pub fn calculate_damage_taken(&self, attack_damage: &AttackDamage) -> i32 {
        let resistance = if attack_damage
            .effects
            .iter()
            .any(|effect| matches!(*effect, AttackEffect::Sharp))
        {
            self.damage_resistance / 2
        } else {
            self.damage_resistance
        };

        // You always take 1 damage from attacks.
        (attack_damage.damage - resistance).max(1)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn from_max(max: i32) -> Self {
        Self { current: max, max }
    }
}
