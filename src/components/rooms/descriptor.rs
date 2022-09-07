#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
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
    Freezing,
    Steamy,
    Sweltering,
}

impl Descriptor {
    pub fn can_be_used_with(&self, other: &Descriptor) -> bool {
        match self {
            Descriptor::Chill => matches!(
                other,
                Descriptor::Dark
                    | Descriptor::Dim
                    | Descriptor::Grimy
                    | Descriptor::Moist
                    | Descriptor::Freezing
            ),
            Descriptor::Dark => matches!(
                other,
                Descriptor::Dark
                    | Descriptor::Grimy
                    | Descriptor::Moist
                    | Descriptor::Freezing
                    | Descriptor::Sweltering
                    | Descriptor::Steamy
            ),
            Descriptor::Dim => matches!(
                other,
                Descriptor::Grimy
                    | Descriptor::Moist
                    | Descriptor::Freezing
                    | Descriptor::Sweltering
                    | Descriptor::Steamy
            ),
            Descriptor::Grimy => true,
            Descriptor::Moist => matches!(
                other,
                Descriptor::Dark
                    | Descriptor::Dim
                    | Descriptor::Grimy
                    | Descriptor::Moist
                    | Descriptor::Sweltering
                    | Descriptor::Steamy
            ),
            Descriptor::Freezing => matches!(
                other,
                Descriptor::Dark | Descriptor::Grimy | Descriptor::Freezing
            ),
            Descriptor::Steamy => matches!(
                other,
                Descriptor::Dark
                    | Descriptor::Grimy
                    | Descriptor::Moist
                    | Descriptor::Sweltering
                    | Descriptor::Steamy
            ),
            Descriptor::Sweltering => matches!(
                other,
                Descriptor::Dark | Descriptor::Grimy | Descriptor::Moist | Descriptor::Steamy
            ),
        }
    }
}
