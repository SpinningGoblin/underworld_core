#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use chrono::{DateTime, Utc};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::spells::Spell;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct PlayerSpellLearned {
    pub spell_id: Uuid,
    pub learned_at: DateTime<Utc>,
    pub spell: Spell,
}
