use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use poem_openapi::Enum;

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug, IntoEnumIterator)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all ="snake_case"))]
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

impl Descriptor {
    pub fn get_position(&self) -> DescriptorPosition {
        match *self {
            Descriptor::Chill => DescriptorPosition::Pre,
            Descriptor::Dark => DescriptorPosition::Pre,
            Descriptor::Dim => DescriptorPosition::Pre,
            Descriptor::Grimy => DescriptorPosition::Pre,
            Descriptor::Moist => DescriptorPosition::Pre,
        }
    }

    pub fn is_pre(&self) -> bool {
        self.get_position() == DescriptorPosition::Pre
    }
}
