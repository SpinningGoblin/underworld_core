#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    games::game_state::GameState,
    items::{character_item::CharacterItem, location_tag::LocationTag},
    player::PlayerCharacter,
};

use super::{
    dead_npc_beaten::DeadNpcBeaten, fixture_can_be_opened_discovered::FixtureCanBeOpenedDiscovered,
    fixture_contained_discovered::FixtureContainedDiscovered,
    fixture_has_hidden_discovered::FixtureHasHiddenDiscovered,
    fixture_hidden_items_discovered::FixtureHiddenItemsDiscovered, fixture_viewed::FixtureViewed,
    item_taken_from_fixture::ItemTakenFromFixture, item_taken_from_npc::ItemTakenFromNpc,
    npc_health_discovered::NpcHealthDiscovered, npc_hidden_discovered::NpcHiddenDiscovered,
    npc_hit::NpcHit, npc_killed::NpcKilled, npc_missed::NpcMissed,
    npc_name_discovered::NpcNameDiscovered, npc_packed_discovered::NpcPackedDiscovered,
    npc_viewed::NpcViewed, npc_weapon_readied::NpcWeaponReadied,
    player_gains_resurrection_aura::PlayerGainsResurrectionAura,
    player_gains_retribution_aura::PlayerGainsRetributionAura,
    player_gains_shield_aura::PlayerGainsShieldAura, player_healed::PlayerHealed,
    player_hit::PlayerHit, player_item_moved::PlayerItemMoved, player_killed::PlayerKilled,
    player_missed::PlayerMissed, player_resurrected::PlayerResurrected, room_exited::RoomExited,
    room_first_seen::RoomFirstSeen, room_generated::RoomGenerated, room_viewed::RoomViewed, player_retribution_aura_dissipated::PlayerRetributionAuraDissipated,
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
    ItemTakenFromFixture(ItemTakenFromFixture),
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
    PlayerGainsResurrectionAura(PlayerGainsResurrectionAura),
    PlayerGainsRetributionAura(PlayerGainsRetributionAura),
    PlayerGainsShieldAura(PlayerGainsShieldAura),
    PlayerHealed(PlayerHealed),
    PlayerHit(PlayerHit),
    PlayerItemMoved(PlayerItemMoved),
    PlayerKilled(PlayerKilled),
    PlayerMissed(PlayerMissed),
    PlayerResurrected(PlayerResurrected),
    PlayerRetributionAuraDissipated(PlayerRetributionAuraDissipated),
    RoomExited(RoomExited),
    RoomGenerated(RoomGenerated),
    RoomFirstSeen(RoomFirstSeen),
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
            }
            Event::RoomGenerated(room_generated) => new_game
                .world
                .add_room(room_generated.entrance_id, room_generated.room.clone()),
            Event::NpcHit(npc_hit) => {
                if let Some(position) = new_game.current_room_mut().find_npc_mut(&npc_hit.npc_id) {
                    position.npc.character.damage(npc_hit.damage);
                }
            }
            Event::NpcKilled(npc_killed) => {
                let room = new_game.current_room_mut();
                if let Some(position) = room.find_npc_mut(&npc_killed.npc_id) {
                    position.npc.character.kill();
                    position.position_descriptor = None;
                }
            }
            Event::PlayerHit(player_hit) => {
                new_player.character.damage(player_hit.damage);
            }
            Event::PlayerKilled(_) => new_player.character.kill(),
            Event::ItemTakenFromNpc(item_taken_from_npc) => {
                let position = new_game
                    .current_room_mut()
                    .find_npc_mut(&item_taken_from_npc.npc_id)
                    .unwrap();
                let character_item = position
                    .npc
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
            Event::NpcWeaponReadied(weapon_readied) => {
                let position = new_game
                    .current_room_mut()
                    .find_npc_mut(&weapon_readied.npc_id)
                    .unwrap();
                let mut character_item = position
                    .npc
                    .character
                    .remove_item(&weapon_readied.item_id)
                    .unwrap();
                character_item.at_the_ready = true;
                character_item.equipped_location = LocationTag::Hand;
                position.npc.character.add_item(character_item);
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
                knowledge.knows_hidden_in_inventory = true;
                new_game.set_npc_knowledge(hidden_discovered.npc_id, knowledge);
            }
            Event::NpcNameDiscovered(name_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&name_discovered.npc_id);
                knowledge.knows_name = true;
                new_game.set_npc_knowledge(name_discovered.npc_id, knowledge);
            }
            Event::NpcPackedDiscovered(packed_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&packed_discovered.npc_id);
                knowledge.knows_packed_in_inventory = true;
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
            Event::RoomFirstSeen(first_seen) => {
                new_game.rooms_seen.push(first_seen.room_id);
            }
            Event::ItemTakenFromFixture(item_taken_from_fixture) => {
                let fixture = new_game
                    .current_room_mut()
                    .find_fixture_mut(&item_taken_from_fixture.fixture_id)
                    .unwrap();
                let fixture_item = fixture
                    .remove_item(&item_taken_from_fixture.item_id)
                    .unwrap();

                let packed_item = CharacterItem {
                    is_hidden: false,
                    equipped_location: LocationTag::Packed,
                    is_multiple: false,
                    item: fixture_item.item,
                    at_the_ready: false,
                };
                new_player.character.add_item(packed_item)
            }
            Event::PlayerHealed(player_healed) => {
                new_player.character.heal(player_healed.damage_healed)
            }
            Event::PlayerGainsResurrectionAura(_) => {
                new_player.character.current_effects.resurrection_aura = true;
            }
            Event::PlayerGainsRetributionAura(gain_retribution_aura) => {
                new_player.character.current_effects.retribution_aura =
                    Some(gain_retribution_aura.attack.clone())
            }
            Event::PlayerGainsShieldAura(gain_shield_aura) => {
                new_player.character.current_effects.shield_aura =
                    Some(gain_shield_aura.defense.clone())
            }
            Event::PlayerResurrected(_) => {
                new_player.character.heal_to_max();
            }
            Event::PlayerRetributionAuraDissipated(_) => {
                new_player.character.current_effects.retribution_aura = None;
            },
            Event::NpcMissed(_)
            | Event::DeadNpcBeaten(_)
            | Event::PlayerMissed(_)
            | Event::NpcViewed(_)
            | Event::FixtureViewed(_)
            | Event::RoomViewed(_) => {}
        }
    }

    (new_game, new_player)
}
