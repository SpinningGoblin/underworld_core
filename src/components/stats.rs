#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{dimensions::Dimensions, health::Health};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Stats {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub health: Option<Health>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub dimensions: Option<Dimensions>,
}
