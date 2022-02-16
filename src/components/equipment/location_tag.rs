use enum_iterator::IntoEnumIterator;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
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
