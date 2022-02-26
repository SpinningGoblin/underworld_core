use enum_iterator::IntoEnumIterator;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum LocationTag {
    Ankle,
    Arm,
    Back,
    Body,
    Equipped,
    Feet,
    Hand,
    Head,
    Hip,
    HipSheath,
    Leg,
    Neck,
    Packed,
    Pockets,
    Shoulder,
    Waist,
    Wrist,
}

impl LocationTag {
    pub fn hides_full_item(&self) -> bool {
        matches!(
            *self,
            LocationTag::HipSheath | LocationTag::Packed | LocationTag::Pockets
        )
    }
}
