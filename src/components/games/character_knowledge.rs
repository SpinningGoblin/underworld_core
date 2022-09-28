#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct CharacterKnowledge {
    pub knows_health: bool,
    pub knows_species: bool,
    pub knows_life_modifier: bool,
    pub knows_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl Default for CharacterKnowledge {
    fn default() -> Self {
        Self {
            knows_health: false,
            knows_species: true,
            knows_life_modifier: true,
            knows_inventory: true,
            knows_packed_in_inventory: false,
        }
    }
}
