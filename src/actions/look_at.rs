#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::character::CharacterViewArgs;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct LookAtTarget {
    pub room_id: String,
    pub target: String,
}

impl LookAtTarget {
    pub fn description(&self) -> String {
        "Look at a target inside of a room".to_string()
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct LookAtNpc {
    pub npc_id: String,
    pub args: CharacterViewArgs,
    pub knows_name: bool,
    pub knows_all: bool,
}
