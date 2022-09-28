use std::collections::HashMap;

use crate::components::{
    games::{game_state::GameStateView, GameState},
    worlds::{ExitMapView, WorldView},
};

pub fn view(game_state: &GameState) -> GameStateView {
    let exit_graph = game_state
        .world
        .exit_graph
        .iter()
        .map(|exit_map| ExitMapView {
            exit_id: exit_map.exit_id.to_string(),
            left_room_id: exit_map.left_room_id.map(|id| id.to_string()),
            right_room_id: exit_map.right_room_id.map(|id| id.to_string()),
        })
        .collect();

    let rooms = game_state
        .world
        .rooms
        .iter()
        .map(|room| game_state.view_room(room))
        .collect();

    let rooms_seen = game_state
        .rooms_seen
        .iter()
        .map(|room_id| room_id.to_string())
        .collect();

    let player_npc_knowledge = game_state
        .player_npc_knowledge
        .iter()
        .map(|(npc_id, knowledge)| (npc_id.to_string(), knowledge.clone()));

    let player_fixture_knowledge = game_state
        .player_fixture_knowledge
        .iter()
        .map(|(fixture_id, knowledge)| (fixture_id.to_string(), knowledge.clone()));

    let player_statistics = game_state
        .player_statistics
        .iter()
        .map(|(id, stats)| (id.to_string(), stats.clone()));

    GameStateView {
        id: game_state.id.to_string(),
        name: game_state.name.clone(),
        world: WorldView { rooms, exit_graph },
        current_room_id: game_state.current_room_id.to_string(),
        rooms_seen,
        all_knowledge_unlocked: game_state.all_knowledge_unlocked,
        player_npc_knowledge: HashMap::from_iter(player_npc_knowledge),
        player_fixture_knowledge: HashMap::from_iter(player_fixture_knowledge),
        player_statistics: HashMap::from_iter(player_statistics),
        danger_level: game_state.danger_level,
    }
}
