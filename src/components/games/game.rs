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
    pub current_room_id: Uuid,
    pub rooms_seen: Vec<Uuid>,
}

impl Game {
    pub fn current_room_exits(&self) -> Vec<Uuid> {
        let current_room = self
            .world
            .rooms
            .iter()
            .find(|room| room.identifier.id.eq(&self.current_room_id))
            .unwrap();
        current_room
            .exits
            .iter()
            .map(|exit| exit.identifier.id)
            .collect()
    }
}
