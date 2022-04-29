#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    games::game_state::GameState,
    items::{character_item::CharacterItem, location_tag::LocationTag},
    non_player::NonPlayer,
    player::PlayerCharacter,
    rooms::npc_position::NpcPosition,
};

use super::{
    dead_npc_beaten::DeadNpcBeaten, item_taken_from_npc::ItemTakenFromNpc, npc_hit::NpcHit,
    npc_killed::NpcKilled, npc_missed::NpcMissed, npc_viewed::NpcViewed, player_hit::PlayerHit,
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
    DeadNpcBeaten(DeadNpcBeaten),
    ItemTakenFromNpc(ItemTakenFromNpc),
    NpcHit(NpcHit),
    NpcKilled(NpcKilled),
    NpcMissed(NpcMissed),
    NpcViewed(NpcViewed),
    PlayerHit(PlayerHit),
    PlayerKilled(PlayerKilled),
    PlayerMissed(PlayerMissed),
    RoomExited(RoomExited),
    RoomGenerated(RoomGenerated),
}

pub fn apply_events(
    events: &[Event],
    state: &GameState,
    player: &PlayerCharacter,
) -> (GameState, PlayerCharacter) {
    let mut new_game = state.clone();
    let mut new_player = player.clone();

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
                let room = new_game.current_room_mut();
                if let Some(npc) = room.find_npc_mut(&npc_killed.npc_id) {
                    npc.character.kill();
                }

                if let Some(index) = room.index_of_npc_position(&npc_killed.npc_id) {
                    let position = room.remove_npc_position(index);

                    let (dead, alive): (Vec<NonPlayer>, Vec<NonPlayer>) = position
                        .npcs
                        .into_iter()
                        .partition(|npc| npc.character.is_dead());

                    let replace_alive = !alive.is_empty();
                    if !dead.is_empty() {
                        let dead_position = NpcPosition {
                            group_descriptor: crate::generators::rooms::npcs::group_descriptor(
                                dead.len(),
                            ),
                            npcs: dead,
                            position_descriptor: None,
                        };
                        room.npc_positions.push(dead_position);
                    }

                    if replace_alive {
                        let alive_position = NpcPosition {
                            group_descriptor: crate::generators::rooms::npcs::group_descriptor(
                                alive.len(),
                            ),
                            npcs: alive,
                            position_descriptor: position.position_descriptor.clone(),
                        };
                        room.npc_positions.push(alive_position);
                    }
                }
            }
            Event::PlayerHit(player_hit) => {
                new_player.character.damage(player_hit.damage);
            }
            Event::PlayerKilled(_) => new_player.character.kill(),
            Event::PlayerMissed(_) => {}
            Event::NpcViewed(_) => {}
            Event::ItemTakenFromNpc(item_taken_from_npc) => {
                let npc = new_game
                    .current_room_mut()
                    .find_npc_mut(&item_taken_from_npc.npc_id)
                    .unwrap();
                let character_item = npc
                    .character
                    .remove_item(&item_taken_from_npc.item_id)
                    .unwrap();

                let packed_item = CharacterItem {
                    is_hidden: false,
                    equipped_location_tags: vec![LocationTag::Packed],
                    is_multiple: character_item.is_multiple,
                    item: character_item.item,
                    at_the_ready: false,
                };
                new_player.character.add_item(packed_item)
            }
            Event::DeadNpcBeaten(_) => {}
        }
    }

    (new_game, new_player)
}
