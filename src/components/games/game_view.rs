#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    identifier::IdentifierView, player::PlayerCharacterView, worlds::world_view::WorldView,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct GameView {
    pub identifier: IdentifierView,
    pub world: WorldView,
    pub player: PlayerCharacterView,
    pub current_room: String,
}
