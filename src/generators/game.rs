use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    games::game_state::GameState,
    rooms::room_type::RoomType,
    worlds::world::{ExitMap, World},
};

use super::{generator::Generator, rooms::room_generator};

pub fn game_generator() -> impl Generator<GameState> {
    GamePrototype {}
}

struct GamePrototype {}

impl Generator<GameState> for GamePrototype {
    fn generate(&self) -> GameState {
        let entry = room_generator(&RoomType::EntryWay, None).generate();

        let exit_maps: Vec<ExitMap> = entry
            .exits
            .iter()
            .map(|exit| ExitMap {
                exit_id: exit.id,
                left_room_id: Some(entry.id),
                right_room_id: None,
            })
            .collect();

        GameState {
            id: Uuid::new_v4(),
            name: None,
            current_room_id: entry.id,
            rooms_seen: vec![entry.id],
            world: World {
                rooms: vec![entry],
                exit_graph: exit_maps,
            },
            player_knows_all: false,
            player_npc_knowledge: HashMap::new(),
            player_fixture_knowledge: HashMap::new(),
            player_statistics: HashMap::new(),
            danger_level: 1,
        }
    }
}
