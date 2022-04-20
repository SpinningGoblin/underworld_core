use crate::components::{
    games::{game::Game, game_state::GameState},
    identifier::Identifier,
    player::PlayerCharacter,
    rooms::room_type::RoomType,
    worlds::world::{ExitMap, World},
};

use super::{generator::Generator, rooms::room_generator};

pub fn game_generator(player: PlayerCharacter) -> impl Generator<Game> {
    GamePrototype { player }
}

struct GamePrototype {
    pub player: PlayerCharacter,
}

impl Generator<Game> for GamePrototype {
    fn generate(&self) -> Game {
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

        let state = GameState {
            identifier: Identifier::just_id(),
            current_room_id: entry.identifier.id,
            rooms_seen: vec![entry.identifier.id],
            world: World {
                rooms: vec![entry],
                exit_graph: exit_maps,
            },
        };

        Game {
            player: self.player.clone(),
            state,
        }
    }
}
