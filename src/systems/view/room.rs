use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::fixture::FixtureViewArgs,
    non_player::NonPlayerViewArgs,
    rooms::{
        exit::ExitView,
        fixture_position::FixturePositionView,
        npc_position::NpcPositionView,
        room::Room,
        room_view::{RoomView, RoomViewArgs},
    },
};

pub fn look_at(room: &Room, _args: RoomViewArgs, knows_all: bool) -> RoomView {
    view(room, HashMap::new(), HashMap::new(), knows_all)
}

pub fn view(
    room: &Room,
    non_player_args: HashMap<Uuid, NonPlayerViewArgs>,
    fixture_args: HashMap<Uuid, FixtureViewArgs>,
    knows_all: bool,
) -> RoomView {
    let fixture_positions: Vec<FixturePositionView> = room
        .fixture_positions
        .iter()
        .map(|fixture_position| {
            super::fixture_position::view_v2(fixture_position, &fixture_args, knows_all)
        })
        .into_iter()
        .collect();
    let npc_positions: Vec<NpcPositionView> = room
        .npc_positions
        .iter()
        .map(|npc_position| super::npc_position::view_v2(npc_position, &non_player_args, knows_all))
        .into_iter()
        .collect();

    let exits: Vec<ExitView> = room
        .exits
        .iter()
        .map(super::exit::look_at)
        .into_iter()
        .collect();

    RoomView {
        identifier: super::identifier::to_view(&room.identifier, true),
        descriptors: room.descriptors.clone(),
        room_type: room.room_type.clone(),
        fixture_positions,
        dimensions: room.dimensions.clone(),
        npc_positions,
        flavour: room.flavour.clone(),
        exits,
    }
}
