#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use chrono::{DateTime, Utc};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{identifier::Identifier, spells::spell::Spell};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct PlayerSpellLearned {
    pub spell_identifier: Identifier,
    pub learned_at: DateTime<Utc>,
    pub spell: Spell,
}
