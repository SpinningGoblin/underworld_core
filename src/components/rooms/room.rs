#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::identifier::Identifier;

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
