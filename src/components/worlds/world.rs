#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::rooms::Room;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct World {
    pub rooms: Vec<Room>,
    pub exit_graph: Vec<ExitMap>,
}

impl World {
    pub fn add_room(&mut self, entrance_id: Uuid, room: Room) {
        if let Some(exit_map) = self
            .exit_graph
            .iter_mut()
            .find(|exit_map| exit_map.exit_id.eq(&entrance_id))
        {
            exit_map.set_room_id(room.id);

            room.exits
                .iter()
                .filter(|exit| exit.id.ne(&entrance_id))
                .map(|exit| ExitMap {
                    exit_id: exit.id,
                    left_room_id: Some(room.id),
                    right_room_id: None,
                })
                .for_each(|exit_map| self.exit_graph.push(exit_map));
            self.rooms.push(room);
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct ExitMap {
    pub exit_id: Uuid,
    pub left_room_id: Option<Uuid>,
    pub right_room_id: Option<Uuid>,
}

impl ExitMap {
    pub fn other_room_id(&self, room_id: Uuid) -> Option<Uuid> {
        if self.left_room_id.eq(&Some(room_id)) {
            self.right_room_id
        } else {
            self.left_room_id
        }
    }

    pub fn set_room_id(&mut self, room_id: Uuid) {
        if self.left_room_id.is_none() {
            self.left_room_id = Some(room_id);
        } else if self.right_room_id.is_none() {
            self.right_room_id = Some(room_id);
        }
    }
}
