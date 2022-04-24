use rand::Rng;

use crate::{
    actions::attack_npc::AttackNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Errors,
    events::{event::Event, npc_hit::NpcHit, npc_killed::NpcKilled, player_missed::PlayerMissed},
    utils::ids::parse_id,
};

use super::helpers::npc_attack_player;

const PLAYER_DODGE_CHANCE: usize = 1;

pub fn handle_attack_npc(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Errors> {
    let mut events: Vec<Event> = Vec::new();

    let room = state.current_room();
    let npc_id = parse_id(&attack_npc.npc_id)?;

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Errors::NpcNotFound(npc_id.to_string())),
    };

    let defense = npc.character.defense();
    let attack = player.character.attack();
    let damage = (attack - defense).max(1);
    events.push(Event::NpcHit(NpcHit {
        npc_id,
        damage,
        attacker_id: player.identifier.id,
    }));

    if damage > npc.character.get_current_health().unwrap() {
        events.push(Event::NpcKilled(NpcKilled {
            npc_id,
            killer_id: player.identifier.id,
        }));
    } else {
        let mut rng = rand::thread_rng();
        let dodge_roll = rng.gen_range(1..=6);

        if dodge_roll <= PLAYER_DODGE_CHANCE {
            events.push(Event::PlayerMissed(PlayerMissed {
                attacker_id: npc.identifier.id,
            }));
        } else {
            events.append(&mut npc_attack_player(player, npc));
        }
    }

    Ok(events)
}
