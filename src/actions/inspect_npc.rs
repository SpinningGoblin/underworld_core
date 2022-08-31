#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object))]
/// Inspect an NPC, with a chance to reveal more information
/// than was previously known about the NPC.
pub struct InspectNpc {
    pub npc_id: String,
    /// Attempt to discover the NPC's health.
    pub discover_health: bool,
    /// Attempt to discover the items the NPC has packed away.
    pub discover_packed_items: bool,
}
