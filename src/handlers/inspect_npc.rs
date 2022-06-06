use crate::{
    actions::InspectNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::{Error, NpcNotFoundError},
    events::{Event, NpcHealthDiscovered, NpcHiddenDiscovered, NpcPackedDiscovered},
    utils::{ids::parse_id, rolls::roll_d6},
};

use super::helpers::npc_attack_player;

const DISCOVER_HEALTH_CHANCE: i32 = 5;
const DISCOVER_PACKED_CHANCE: i32 = 4;
const DISCOVER_HIDDEN_CHANCE: i32 = 2;
const NPC_ATTACKS_CHANCE: i32 = 5;

pub fn handle(
    inspect_npc: &InspectNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = Vec::new();
    let npc_id = parse_id(&inspect_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => {
            return Err(Error::NpcNotFoundError(NpcNotFoundError(
                npc_id.to_string(),
            )))
        }
    };

    if npc.character.is_dead() {
        events.push(Event::NpcHealthDiscovered(NpcHealthDiscovered { npc_id }));
        events.push(Event::NpcPackedDiscovered(NpcPackedDiscovered { npc_id }));
        events.push(Event::NpcHiddenDiscovered(NpcHiddenDiscovered { npc_id }));
    } else {
        let mut rng = rand::thread_rng();

        if inspect_npc.discover_health && roll_d6(&mut rng, 1, 0) >= DISCOVER_HEALTH_CHANCE {
            events.push(Event::NpcHealthDiscovered(NpcHealthDiscovered { npc_id }));
        }

        if inspect_npc.discover_packed_items && roll_d6(&mut rng, 1, 0) >= DISCOVER_PACKED_CHANCE {
            events.push(Event::NpcPackedDiscovered(NpcPackedDiscovered { npc_id }));
        }

        if inspect_npc.discover_hidden_items && roll_d6(&mut rng, 1, 0) >= DISCOVER_HIDDEN_CHANCE {
            events.push(Event::NpcHiddenDiscovered(NpcHiddenDiscovered { npc_id }));
        }

        if roll_d6(&mut rng, 1, 0) >= NPC_ATTACKS_CHANCE {
            events.append(&mut npc_attack_player(player, npc));
        }
    }

    Ok(events)
}
