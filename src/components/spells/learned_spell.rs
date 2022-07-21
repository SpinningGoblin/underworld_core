#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use chrono::{DateTime, Utc};
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Spell, SpellView};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct LearnedSpell {
    pub id: Uuid,
    pub spell: Spell,
    pub learned_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "LearnedSpell"))]
pub struct LearnedSpellView {
    pub id: String,
    pub spell: SpellView,
    pub learned_at: String,
}
