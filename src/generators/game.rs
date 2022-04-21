use crate::components::{
    games::game_state::GameState,
    identifier::Identifier,
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
                exit_id: exit.identifier.id,
                left_room_id: Some(entry.identifier.id),
                right_room_id: None,
            })
            .collect();

        GameState {
            identifier: Identifier::just_id(),
            current_room_id: entry.identifier.id,
            rooms_seen: vec![entry.identifier.id],
            world: World {
                rooms: vec![entry],
                exit_graph: exit_maps,
            },
        }
    }
}
