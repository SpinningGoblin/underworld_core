use std::fmt::Display;

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
pub enum RoomDescriptor {
    Chill,
    Dark,
    Dim,
    Grimy,
    Moist,
}

impl Display for RoomDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RoomDescriptor::Chill => write!(f, "chill"),
            RoomDescriptor::Dark => write!(f, "dark"),
            RoomDescriptor::Dim => write!(f, "dim"),
            RoomDescriptor::Grimy => write!(f, "grimy"),
            RoomDescriptor::Moist => write!(f, "moist"),
        }
    }
}
