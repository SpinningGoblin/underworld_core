#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::identifier::Identifier;

use super::{
    descriptor::Descriptor, dimensions::Dimensions, fixture_position::FixturePosition,
    flavour::Flavour, npc_position::NpcPosition, room_type::RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Room {
    pub identifier: Identifier,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub fixture_positions: Vec<FixturePosition>,
    pub dimensions: Dimensions,
    pub npc_positions: Vec<NpcPosition>,
    pub flavour: Option<Flavour>,
}

impl Room {}
