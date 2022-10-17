#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{items::Item, NonPlayer};

use super::{Descriptor, Dimensions, Exit, FixturePosition, Flavour, NpcPosition, RoomType};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Room {
    pub id: Uuid,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub name: Option<String>,
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
    #[cfg_attr(feature = "serialization", serde(default))]
    pub loose_items: Vec<Item>,
}

impl Room {
    pub fn find_npc(&self, npc_id: &Uuid) -> Option<&NonPlayer> {
        self.npc_positions
            .iter()
            .map(|npc_position| &npc_position.npc)
            .find(|npc| npc.id.eq(npc_id))
    }

    pub fn find_fixture(&self, fixture_id: &Uuid) -> Option<&FixturePosition> {
        self.fixture_positions
            .iter()
            .find(|fixture_position| fixture_position.fixture.id.eq(fixture_id))
    }

    pub fn first_alive_npc(&self) -> Option<&NonPlayer> {
        self.npc_positions
            .iter()
            .map(|npc_position| &npc_position.npc)
            .filter(|npc| !npc.character.is_dead())
            .find(|_| true) // First one
    }

    pub fn find_npc_mut(&mut self, target_id: &Uuid) -> Option<&mut NpcPosition> {
        self.npc_positions
            .iter_mut()
            .find(|npc_position| npc_position.npc.id.eq(target_id))
    }

    pub fn find_fixture_mut(&mut self, fixture_id: &Uuid) -> Option<&mut FixturePosition> {
        self.fixture_positions
            .iter_mut()
            .find(|fixture_position| fixture_position.fixture.id.eq(fixture_id))
    }

    pub fn remove_loose_item(&mut self, item_id: &Uuid) -> Option<Item> {
        let index = self
            .loose_items
            .iter()
            .enumerate()
            .find(|(_, item)| item.id.eq(item_id))
            .map(|(index, _)| index);
        index.map(|i| self.loose_items.remove(i))
    }
}
