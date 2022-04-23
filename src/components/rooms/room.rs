#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{identifier::Identifier, non_player::NonPlayer};

use super::{
    descriptor::Descriptor, dimensions::Dimensions, exit::Exit, fixture_position::FixturePosition,
    flavour::Flavour, npc_position::NpcPosition, room_type::RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Room {
    pub identifier: Identifier,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub fixture_positions: Vec<FixturePosition>,
    pub dimensions: Dimensions,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub npc_positions: Vec<NpcPosition>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub flavour: Option<Flavour>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub exits: Vec<Exit>,
}

impl Room {
    pub fn find_npc(&self, target_id: &Uuid) -> Option<&NonPlayer> {
        self.npc_positions
            .iter()
            .flat_map(|npc_position| npc_position.npcs.iter())
            .find(|npc| npc.identifier.id.eq(target_id))
    }

    pub fn find_npc_mut(&mut self, target_id: &Uuid) -> Option<&mut NonPlayer> {
        self.npc_positions
            .iter_mut()
            .flat_map(|npc_position| npc_position.npcs.iter_mut())
            .find(|npc| npc.identifier.id.eq(target_id))
    }

    pub fn index_of_npc_position(&self, npc_id: &Uuid) -> Option<usize> {
        self.npc_positions
            .iter()
            .enumerate()
            .find(|(_, npc_position)| {
                npc_position
                    .npcs
                    .iter()
                    .any(|npc| npc.identifier.id.eq(npc_id))
            })
            .map(|(index, _)| index)
    }

    pub fn remove_npc_position(&mut self, index: usize) -> NpcPosition {
        self.npc_positions.remove(index)
    }
}
