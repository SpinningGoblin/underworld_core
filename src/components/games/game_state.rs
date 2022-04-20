#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{identifier::Identifier, rooms::room::Room, worlds::world::World};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct GameState {
    pub identifier: Identifier,
    pub world: World,
    pub current_room_id: Uuid,
    pub rooms_seen: Vec<Uuid>,
}

impl GameState {
    pub fn current_room_exits(&self) -> Vec<Uuid> {
        self.current_room()
            .exits
            .iter()
            .map(|exit| exit.identifier.id)
            .collect()
    }

    pub fn current_room(&self) -> &Room {
        self.world
            .rooms
            .iter()
            .find(|room| room.identifier.id.eq(&self.current_room_id))
            .unwrap()
    }

    pub fn current_room_mut(&mut self) -> &mut Room {
        self.world
            .rooms
            .iter_mut()
            .find(|room| room.identifier.id.eq(&self.current_room_id))
            .unwrap()
    }
}
