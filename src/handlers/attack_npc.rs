use crate::{
    actions::AttackNpc,
    components::{games::GameState, player::PlayerCharacter, species::Species},
    errors::Error,
    events::{DeadNpcBeaten, Event, NpcMissed},
    utils::{ids::parse_id, rolls::roll_d100},
};

use super::helpers::damage_npc;

pub fn handle(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = Vec::new();

    let room = state.current_room();
    let npc_id = parse_id(&attack_npc.npc_id)?;

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    if npc.character.is_dead() {
        events.push(Event::DeadNpcBeaten(DeadNpcBeaten {
            attacker_id: player.id,
            npc_id,
        }));
    } else if npc_will_dodge(&npc.character.species) {
        events.push(Event::NpcMissed(NpcMissed {
            attacker_id: player.id,
            npc_id,
        }));
    } else {
        let defense = npc.character.defense();
        let attack = player.character.attack();
        let attack_damage = (attack - defense).max(1);
        let damage = attack_damage.min(npc.character.get_current_health());

        events.append(&mut damage_npc(player, npc, damage));
    }

    Ok(events)
}

const PHANTOM_DODGE_CHANCE: i32 = 15;
const SHADOW_DODGE_CHANCE: i32 = 25;

fn npc_will_dodge(species: &Species) -> bool {
    let mut rng = rand::thread_rng();
    let dodge_roll = roll_d100(&mut rng, 1, 0);

    match *species {
        Species::Phantom => dodge_roll <= PHANTOM_DODGE_CHANCE,
        Species::Shadow => dodge_roll <= SHADOW_DODGE_CHANCE,
        _ => false,
    }
}
