use std::error::Error;

use crate::{
    actions::Action,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::PlayerIsDeadError,
    events::event::{apply_events, Event},
};

pub struct HandledAction {
    pub events: Vec<Event>,
    pub new_player: PlayerCharacter,
    pub new_state: GameState,
}

pub fn handle_action(
    action: &Action,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<HandledAction, Box<dyn Error>> {
    let not_viewing_room = match &action {
        Action::LookAtCurrentRoom(_) => false,
        _ => true,
    };

    if player.character.is_dead() && not_viewing_room {
        // TODO: Later we might want more specific handling for this,
        // where maybe some things could happen with a dead player.
        return Err(Box::new(PlayerIsDeadError));
    }

    let events = match action {
        Action::ExitRoom(exit_room) => super::exit_room::handle(exit_room, state)?,
        Action::AttackNpc(attack_npc) => super::attack_npc::handle(attack_npc, state, player)?,
        Action::LootNpc(loot_npc) => super::loot_npc::handle(loot_npc, state, player)?,
        Action::LookAtNpc(look_at_npc) => super::view_npc::handle(look_at_npc, state)?,
        Action::MovePlayerItem(move_player_item) => {
            super::move_player_item::handle(move_player_item, player)?
        }
        Action::LookAtCurrentRoom(look_at_current_room) => {
            super::view_room::handle(look_at_current_room, state)?
        }
        Action::InspectNpc(inspect_npc) => super::inspect_npc::handle(inspect_npc, state, player)?,
        Action::InspectFixture(inspect_fixture) => {
            super::inspect_fixture::handle(inspect_fixture, state, player)?
        }
        Action::LookAtFixture(look_at_fixture) => {
            super::view_fixture::handle(look_at_fixture, state)?
        }
        Action::LootFixture(loot_fixture) => super::loot_fixture::handle(loot_fixture, state)?,
        Action::CastSpellOnNpc(cast_spell_on_npc) => {
            super::cast_spell_on_npc::handle(cast_spell_on_npc, state, player)?
        }
        Action::CastSpellOnPlayer(cast_spell_on_player) => {
            super::cast_spell_on_player::handle(cast_spell_on_player, player)?
        }
        Action::UseItemOnPlayer(use_item_on_player) => {
            super::use_item_on_player::handle(use_item_on_player, player)?
        }
    };

    let (new_state, new_player) = apply_events(&events, state, player);

    Ok(HandledAction {
        new_state,
        new_player,
        events,
    })
}
