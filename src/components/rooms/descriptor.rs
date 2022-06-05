#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug, EnumIter)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "openapi",
    derive(Enum),
    oai(rename_all = "snake_case", rename = "RoomDescriptor")
)]
pub enum Descriptor {
    Chill,
    Dark,
    Dim,
    Grimy,
    Moist,
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
