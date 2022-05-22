use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::fixture::FixtureViewArgs,
    non_player::NonPlayerViewArgs,
    rooms::{
        exit::ExitView, fixture_position::FixturePositionView, npc_position::NpcPositionView,
        room::Room, room_view::RoomView,
    },
};

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
            super::fixture_position::view(fixture_position, &fixture_args, knows_all)
        })
        .into_iter()
        .collect();
    let npc_positions: Vec<NpcPositionView> = room
        .npc_positions
        .iter()
        .map(|npc_position| super::npc_position::view(npc_position, &non_player_args, knows_all))
        .into_iter()
        .collect();

    let exits: Vec<ExitView> = room
        .exits
        .iter()
        .map(super::exit::view)
        .into_iter()
        .collect();

    RoomView {
        identifier: super::identifier::view(&room.identifier, true),
        descriptors: room.descriptors.clone(),
        room_type: room.room_type.clone(),
        fixture_positions,
        dimensions: room.dimensions.clone(),
        npc_positions,
        flavour: room.flavour.clone(),
        exits,
    }
}
