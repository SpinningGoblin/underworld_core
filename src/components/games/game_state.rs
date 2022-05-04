use std::collections::HashMap;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{identifier::Identifier, rooms::room::Room, worlds::world::World};

use super::character_knowledge::CharacterKnowledge;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct GameState {
    pub identifier: Identifier,
    pub world: World,
    pub current_room_id: Uuid,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub rooms_seen: Vec<Uuid>,
    pub player_knows_all: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub player_npc_knowledge: HashMap<Uuid, CharacterKnowledge>,
}

impl GameState {
    pub fn npc_knowledge(&self, npc_id: &Uuid) -> CharacterKnowledge {
        self.player_npc_knowledge
            .get(npc_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_npc_knowledge(&mut self, npc_id: Uuid, knowledge: CharacterKnowledge) {
        self.player_npc_knowledge.insert(npc_id, knowledge);
    }

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
