use crate::components::{
    games::game::Game,
    player::PlayerCharacter,
    rooms::room_type::RoomType,
    worlds::world::{ExitMap, World}, identifier::Identifier,
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
        let entry = room_generator(&RoomType::EntryWay).generate();

        let exit_maps: Vec<ExitMap> = entry
            .exits
            .iter()
            .map(|exit| ExitMap {
                exit_id: exit.identifier.id.clone(),
                left_room_id: Some(entry.identifier.id.clone()),
                right_room_id: None,
            })
            .collect();

        Game {
            identifier: Identifier::just_id(),
            current_room: entry.identifier.id.clone(),
            rooms_seen: vec![entry.identifier.id.clone()],
            world: World {
                rooms: vec![entry.clone()],
                exit_graph: exit_maps,
            },
            player: self.player.clone(),
        }
    }
}
