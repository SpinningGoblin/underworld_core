#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::items::ItemView;

use super::{
    Descriptor, Dimensions, ExitView, FixturePositionView, Flavour, NpcPositionView, RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Room"))]
pub struct RoomView {
    pub id: String,
    pub name: Option<String>,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub fixture_positions: Vec<FixturePositionView>,
    pub dimensions: Dimensions,
    pub npc_positions: Vec<NpcPositionView>,
    pub flavour: Option<Flavour>,
    pub exits: Vec<ExitView>,
    pub loose_items: Vec<ItemView>,
}

#[derive(Clone, Debug, Default)]
pub struct RoomViewArgs {
    pub can_see_hidden: bool,
    pub can_see_packed: bool,
    pub knows_character_health: bool,
    pub knows_fixture_items: bool,
    pub knows_fixture_hidden: bool,
    pub knows_fixture_can_be_opened: bool,
    pub knows_fixture_has_hidden: bool,
}
