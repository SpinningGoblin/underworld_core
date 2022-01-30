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
pub enum Descriptor {
    Chill,
    Dark,
    Dim,
    Grimy,
    Moist,
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Descriptor::Chill => write!(f, "chill"),
            Descriptor::Dark => write!(f, "dark"),
            Descriptor::Dim => write!(f, "dim"),
            Descriptor::Grimy => write!(f, "grimy"),
            Descriptor::Moist => write!(f, "moist"),
        }
    }
}
