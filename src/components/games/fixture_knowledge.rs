#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixtureKnowledge {
    pub knows_items: bool,
    pub knows_can_be_opened: bool,
    pub knows_has_hidden: bool,
    pub knows_hidden_items: bool,
}
