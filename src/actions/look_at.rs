use std::str::FromStr;

use uuid::Uuid;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct LookAtTarget {
    pub room_id: String,
    pub target: String,
}

impl LookAtTarget {
    pub fn target_id(&self) -> Option<Uuid> {
        self.id(&self.target)
    }

    pub fn description(&self) -> String {
        "Look at a target inside of a room".to_string()
    }

    fn id(&self, value: &str) -> Option<Uuid> {
        Uuid::from_str(value).ok()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct LookAtRoom {
    pub room_id: String,
}

impl LookAtRoom {
    pub fn description(&self) -> String {
        "Look at a room".to_string()
    }
}
