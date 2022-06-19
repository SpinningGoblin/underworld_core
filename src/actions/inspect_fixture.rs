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
pub struct InspectFixture {
    pub fixture_id: String,
    /// Attempt to discover any hidden compartments and its contents.
    pub discover_hidden_compartment: bool,
}
