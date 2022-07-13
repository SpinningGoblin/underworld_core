#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{NonPlayer, NonPlayerView};

use super::{group_descriptor::GroupDescriptor, npc_position_descriptor::NpcPositionDescriptor};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct NpcPosition {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub npc: NonPlayer,
    pub position_descriptor: Option<NpcPositionDescriptor>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "NpcPosition"))]
pub struct NpcPositionView {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub npc: NonPlayerView,
    pub position_descriptor: Option<NpcPositionDescriptor>,
}
