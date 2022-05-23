use std::error::Error;

use crate::{
    actions::AttackNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::NpcNotFoundError,
    events::{DeadNpcBeaten, Event},
    utils::ids::parse_id,
};

use super::helpers::damage_npc;

pub fn handle(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();

    let room = state.current_room();
    let npc_id = parse_id(&attack_npc.npc_id)?;

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Box::new(NpcNotFoundError(npc_id.to_string()))),
    };

    if npc.character.is_dead() {
        events.push(Event::DeadNpcBeaten(DeadNpcBeaten {
            attacker_id: player.identifier.id,
            npc_id,
        }));
    } else {
        let defense = npc.character.defense();
        let attack = player.character.attack();
        let damage = (attack - defense).max(1);

        events.append(&mut damage_npc(player, npc, damage, true));
    }

    Ok(events)
}
