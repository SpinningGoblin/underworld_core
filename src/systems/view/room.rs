use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::FixtureViewArgs,
    rooms::{ExitView, FixturePositionView, NpcPositionView, Room, RoomView},
    NonPlayerViewArgs,
};

pub fn view(
    room: &Room,
    non_player_args: HashMap<Uuid, NonPlayerViewArgs>,
    fixture_args: HashMap<Uuid, FixtureViewArgs>,
    exit_visitations: HashMap<Uuid, bool>,
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
        .map(|exit| {
            let has_visited_connected_room =
                exit_visitations.get(&exit.id).cloned().unwrap_or_default();
            super::exit::view(exit, has_visited_connected_room)
        })
        .into_iter()
        .collect();

    RoomView {
        id: room.id.to_string(),
        name: room.name.clone(),
        descriptors: room.descriptors.clone(),
        room_type: room.room_type,
        fixture_positions,
        dimensions: room.dimensions.clone(),
        npc_positions,
        flavour: room.flavour,
        exits,
        loose_items: room
            .loose_items
            .iter()
            .map(|item| super::item::view(item, true, knows_all))
            .collect(),
    }
}
