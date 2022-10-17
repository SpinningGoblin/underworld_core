use uuid::Uuid;

use crate::{
    actions::Action,
    components::{games::GameState, PlayerCharacter},
    errors::Error,
    events::{apply_events, Event, GhostEscapesToTheVoid},
};

use super::NpcAction;

pub struct HandledAction {
    pub events: Vec<Event>,
    pub new_player: PlayerCharacter,
    pub new_state: GameState,
}

fn is_being_targeted(action: &Action, npc_id: &Uuid) -> bool {
    match action {
        Action::AttackNpc(attack_npc) => attack_npc.npc_id.eq(&npc_id.to_string()),
        Action::CastSpellOnNpc(cast_spell) => cast_spell.npc_id.eq(&npc_id.to_string()),
        Action::InspectNpc(inspect_npc) => inspect_npc.npc_id.eq(&npc_id.to_string()),
        _ => false,
    }
}

pub fn handle_action(
    action: &Action,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<HandledAction, Error> {
    if player.character.is_dead() {
        // TODO: Later we might want more specific handling for this,
        // where maybe some things could happen with a dead player.
        return Err(Error::PlayerIsDeadError);
    }

    let mut npc_actions: Vec<NpcAction> = Vec::new();
    for npc_position in state
        .current_room()
        .npc_positions
        .iter()
        .filter(|npc_position| !npc_position.npc.character.is_dead())
    {
        if is_being_targeted(action, &npc_position.npc.id) {
            npc_actions.push(NpcAction::AttackPlayer(npc_position.npc.id));
        }
    }

    npc_actions.append(&mut match action {
        Action::InspectFixture(_)
        | Action::LootFixture(_)
        | Action::LootNpc(_)
        | Action::OpenFixture(_)
        | Action::OpenFixtureHiddenCompartment(_) => {
            if let Some(npc) = state.current_room().first_alive_npc() {
                vec![NpcAction::AttackPlayer(npc.id)]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    });

    let mut events: Vec<Event> = Vec::new();

    // Go through all npc actions and handle those. I'll do them first.
    for npc_action in npc_actions.iter() {
        events.append(&mut super::handle_npc_action(npc_action, state, player)?);
    }

    events.append(&mut match action {
        Action::ExitRoom(exit_room) => super::exit_room::handle(exit_room, state)?,
        Action::AttackNpc(attack_npc) => super::attack_npc::handle(attack_npc, state, player)?,
        Action::LootNpc(loot_npc) => super::loot_npc::handle(loot_npc, state)?,
        Action::LookAtNpc(look_at_npc) => super::view_npc::handle(look_at_npc, state)?,
        Action::MovePlayerItem(move_player_item) => {
            super::move_player_item::handle(move_player_item, player)?
        }
        Action::InspectNpc(inspect_npc) => super::inspect_npc::handle(inspect_npc, state)?,
        Action::InspectFixture(inspect_fixture) => {
            super::inspect_fixture::handle(inspect_fixture, state)?
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
        Action::OpenFixture(open_fixture) => super::open_fixture::handle(open_fixture, state)?,
        Action::OpenFixtureHiddenCompartment(open_fixture_hidden_compartment) => {
            super::open_fixture_hidden_compartment::handle(open_fixture_hidden_compartment, state)?
        }
        Action::SellPlayerItem(sell_player_item) => {
            super::sell_player_item::handle(sell_player_item, player)?
        }
        Action::ThrowItemAtNpc(throw_item_at_npc) => {
            super::throw_item_at_npc::handle(throw_item_at_npc, state, player)?
        }
    });

    let (mut intermediate_state, mut intermediate_player) = apply_events(&events, state, player);

    let mut global_events =
        super::global_effects::handle(&intermediate_state, &intermediate_player);
    (intermediate_state, intermediate_player) =
        apply_events(&global_events, &intermediate_state, &intermediate_player);

    events.append(&mut global_events);

    let mut dead_events = dead_player_events(&intermediate_player);

    let (new_state, new_player) =
        apply_events(&dead_events, &intermediate_state, &intermediate_player);
    events.append(&mut dead_events);

    Ok(HandledAction {
        new_state,
        new_player,
        events,
    })
}

fn dead_player_events(player: &PlayerCharacter) -> Vec<Event> {
    if !player.character.is_dead() {
        return Vec::new();
    }

    vec![
        Event::GhostEscapesToTheVoid(GhostEscapesToTheVoid {
            character: player.character.clone(),
            name: player.name.clone(),
        }),
        Event::PlayerDropsAllItems,
    ]
}
