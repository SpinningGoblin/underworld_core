#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{identifier::Identifier, player::PlayerCharacter, worlds::world::World};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Game {
    pub identifier: Identifier,
    pub world: World,
    pub player: PlayerCharacter,
    pub current_room: Uuid,
    pub rooms_seen: Vec<Uuid>,
}
