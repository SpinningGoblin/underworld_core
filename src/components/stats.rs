#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use super::{dimensions::Dimensions, health::Health};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct Stats {
    pub health: Option<Health>,
    pub dimensions: Option<Dimensions>,
}
