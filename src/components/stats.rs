#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use poem_openapi::Object;

use super::{damage::Health, size::Size};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Stats {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub health: Option<Health>,
    pub height: Size,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct StatsView {
    pub health: Option<Health>,
    pub health_known: bool,
    pub height: Size,
}
