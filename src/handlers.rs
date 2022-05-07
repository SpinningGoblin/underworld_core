use crate::{
    actions::action::Action,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Errors,
    events::event::{apply_events, Event},
};

use self::{
    attack_npc::handle_attack_npc, exit_room::handle_exit_room,
    inspect_fixture::handle_inspect_fixture, inspect_npc::handle_inspect_npc,
    loot_npc::handle_loot_npc, move_player_item::handle_move_player_item,
    view_npc::handle_view_npc,
};

mod attack_npc;
mod exit_room;
mod helpers;
mod inspect_fixture;
mod inspect_npc;
mod loot_npc;
mod move_player_item;
mod view_npc;

pub struct HandledAction {
    pub events: Vec<Event>,
    pub new_player: PlayerCharacter,
    pub new_state: GameState,
}

pub fn handle(
    action: &Action,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<HandledAction, Errors> {
    if player.character.is_dead() {
        /*
        TODO: Later we might want more specific handling for this
        where maybe some things could happen with a dead player.
        */
        return Err(Errors::PlayerIsDead);
    }

    let events = match action {
        Action::ExitRoom(exit_room) => handle_exit_room(exit_room, state)?,
        Action::AttackNpc(attack_npc) => handle_attack_npc(attack_npc, state, player)?,
        Action::LootNpc(loot_npc) => handle_loot_npc(loot_npc, state, player)?,
        Action::LookAtNpc(look_at_npc) => handle_view_npc(look_at_npc, state)?,
        Action::MovePlayerItem(move_player_item) => {
            handle_move_player_item(move_player_item, player)?
        }
        Action::LookAtCurrentRoom(_) => Vec::new(),
        Action::InspectNpc(inspect_npc) => handle_inspect_npc(inspect_npc, state, player)?,
        Action::InspectFixture(inspect_fixture) => {
            handle_inspect_fixture(inspect_fixture, state, player)?
        }
    };

    let (new_state, new_player) = apply_events(&events, state, player);

    Ok(HandledAction {
        new_state,
        new_player,
        events,
    })
}
