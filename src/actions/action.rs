use super::{
    look_at::{LookAtRoom, LookAtTarget},
    quick_look::QuickLookRoom,
};

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
pub enum Action {
    LookAtTarget(LookAtTarget),
    LookAtRoom(LookAtRoom),
    QuickLookRoom(QuickLookRoom),
}
