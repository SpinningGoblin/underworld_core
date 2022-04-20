#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::games::game_state::GameState;

use super::{
    npc_hit::NpcHit, npc_killed::NpcKilled, npc_missed::NpcMissed, player_hit::PlayerHit,
    player_killed::PlayerKilled, player_missed::PlayerMissed, room_exited::RoomExited,
    room_generated::RoomGenerated,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", tag = "event_type")
)]
pub enum Event {
    NpcHit(NpcHit),
    NpcKilled(NpcKilled),
    NpcMissed(NpcMissed),
    PlayerHit(PlayerHit),
    PlayerKilled(PlayerKilled),
    PlayerMissed(PlayerMissed),
    RoomExited(RoomExited),
    RoomGenerated(RoomGenerated),
}

pub fn apply_events(events: &[Event], game: &GameState) -> GameState {
    let mut new_game = game.clone();

    for event in events.iter() {
        match event {
            Event::RoomExited(room_exited) => {
                new_game.current_room_id = room_exited.new_room_id;
                new_game.rooms_seen.push(room_exited.new_room_id);
            }
            Event::RoomGenerated(room_generated) => new_game
                .world
                .add_room(room_generated.entrance_id, room_generated.room.clone()),
            Event::NpcHit(npc_hit) => {
                if let Some(npc) = new_game.current_room_mut().find_npc_mut(&npc_hit.npc_id) {
                    npc.character.damage(npc_hit.damage);
                }
            }
            Event::NpcMissed(_) => {}
            Event::NpcKilled(npc_killed) => {
                if let Some(npc) = new_game.current_room_mut().find_npc_mut(&npc_killed.npc_id) {
                    npc.character.kill();
                }
            }
            Event::PlayerHit(_) => {}
            Event::PlayerKilled(_) => {}
            Event::PlayerMissed(_) => {}
        }
    }

    new_game
}
