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
    dead_npc_beaten::DeadNpcBeaten, fixture_can_be_opened_discovered::FixtureCanBeOpenedDiscovered,
    fixture_contained_discovered::FixtureContainedDiscovered,
    fixture_has_hidden_discovered::FixtureHasHiddenDiscovered,
    fixture_hidden_items_discovered::FixtureHiddenItemsDiscovered, fixture_viewed::FixtureViewed,
    item_taken_from_npc::ItemTakenFromNpc, npc_health_discovered::NpcHealthDiscovered,
    npc_hidden_discovered::NpcHiddenDiscovered, npc_hit::NpcHit, npc_killed::NpcKilled,
    npc_missed::NpcMissed, npc_name_discovered::NpcNameDiscovered,
    npc_packed_discovered::NpcPackedDiscovered, npc_viewed::NpcViewed,
    npc_weapon_readied::NpcWeaponReadied, player_hit::PlayerHit,
    player_item_moved::PlayerItemMoved, player_killed::PlayerKilled, player_missed::PlayerMissed,
    room_exited::RoomExited, room_generated::RoomGenerated, room_viewed::RoomViewed,
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
    FixtureCanBeOpenedDiscovered(FixtureCanBeOpenedDiscovered),
    FixtureContainedDiscovered(FixtureContainedDiscovered),
    FixtureHasHiddenDiscovered(FixtureHasHiddenDiscovered),
    FixtureHiddenItemsDiscovered(FixtureHiddenItemsDiscovered),
    FixtureViewed(FixtureViewed),
    ItemTakenFromNpc(ItemTakenFromNpc),
    NpcHealthDiscovered(NpcHealthDiscovered),
    NpcHiddenDiscovered(NpcHiddenDiscovered),
    NpcHit(NpcHit),
    NpcKilled(NpcKilled),
    NpcMissed(NpcMissed),
    NpcNameDiscovered(NpcNameDiscovered),
    NpcPackedDiscovered(NpcPackedDiscovered),
    NpcViewed(NpcViewed),
    NpcWeaponReadied(NpcWeaponReadied),
    PlayerHit(PlayerHit),
    PlayerItemMoved(PlayerItemMoved),
    PlayerKilled(PlayerKilled),
    PlayerMissed(PlayerMissed),
    RoomExited(RoomExited),
    RoomGenerated(RoomGenerated),
    RoomViewed(RoomViewed),
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
                    equipped_location: LocationTag::Packed,
                    is_multiple: character_item.is_multiple,
                    item: character_item.item,
                    at_the_ready: false,
                };
                new_player.character.add_item(packed_item)
            }
            Event::DeadNpcBeaten(_) => {}
            Event::NpcWeaponReadied(weapon_readied) => {
                let npc = new_game
                    .current_room_mut()
                    .find_npc_mut(&weapon_readied.npc_id)
                    .unwrap();
                let mut character_item =
                    npc.character.remove_item(&weapon_readied.item_id).unwrap();
                character_item.at_the_ready = true;
                character_item.equipped_location = LocationTag::Hand;
                npc.character.add_item(character_item);
            }
            Event::PlayerItemMoved(item_moved) => {
                let mut character_item = new_player
                    .character
                    .remove_item(&item_moved.item_id)
                    .unwrap();
                character_item.at_the_ready = item_moved.at_the_ready;
                character_item.equipped_location = item_moved.location.clone();
                new_player.character.add_item(character_item);
            }
            Event::NpcHealthDiscovered(health_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&health_discovered.npc_id);
                knowledge.knows_health = true;
                new_game.set_npc_knowledge(health_discovered.npc_id, knowledge);
            }
            Event::NpcHiddenDiscovered(hidden_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&hidden_discovered.npc_id);
                knowledge.knows_health = true;
                new_game.set_npc_knowledge(hidden_discovered.npc_id, knowledge);
            }
            Event::NpcNameDiscovered(name_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&name_discovered.npc_id);
                knowledge.knows_health = true;
                new_game.set_npc_knowledge(name_discovered.npc_id, knowledge);
            }
            Event::NpcPackedDiscovered(packed_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&packed_discovered.npc_id);
                knowledge.knows_health = true;
                new_game.set_npc_knowledge(packed_discovered.npc_id, knowledge);
            }
            Event::FixtureCanBeOpenedDiscovered(opened_discovered) => {
                let mut knowledge = new_game.fixture_knowledge(&opened_discovered.fixture_id);
                knowledge.knows_can_be_opened = true;
                new_game.set_fixture_knowledge(opened_discovered.fixture_id, knowledge);
            }
            Event::FixtureContainedDiscovered(contained_discovered) => {
                let mut knowledge = new_game.fixture_knowledge(&contained_discovered.fixture_id);
                knowledge.knows_items = true;
                new_game.set_fixture_knowledge(contained_discovered.fixture_id, knowledge);
            }
            Event::FixtureHasHiddenDiscovered(has_hidden) => {
                let mut knowledge = new_game.fixture_knowledge(&has_hidden.fixture_id);
                knowledge.knows_has_hidden = true;
                new_game.set_fixture_knowledge(has_hidden.fixture_id, knowledge);
            }
            Event::FixtureHiddenItemsDiscovered(hidden_items) => {
                let mut knowledge = new_game.fixture_knowledge(&hidden_items.fixture_id);
                knowledge.knows_hidden_items = true;
                new_game.set_fixture_knowledge(hidden_items.fixture_id, knowledge);
            }
            Event::FixtureViewed(_) => {}
            Event::RoomViewed(_) => {}
        }
    }

    (new_game, new_player)
}
