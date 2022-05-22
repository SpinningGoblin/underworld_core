use std::error::Error;

use crate::{
    actions::inspect_npc::InspectNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::npc_not_found_error::NpcNotFoundError,
    events::{
        event::Event, npc_health_discovered::NpcHealthDiscovered,
        npc_hidden_discovered::NpcHiddenDiscovered, npc_name_discovered::NpcNameDiscovered,
        npc_packed_discovered::NpcPackedDiscovered,
    },
    utils::{ids::parse_id, rolls::roll_d6},
};

use super::helpers::npc_attack_player;

const DISCOVER_HEALTH_CHANCE: i32 = 5;
const DISCOVER_NAME_CHANCE: i32 = 6;
const DISCOVER_PACKED_CHANCE: i32 = 4;
const DISCOVER_HIDDEN_CHANCE: i32 = 2;
const NPC_ATTACKS_CHANCE: i32 = 5;

pub fn handle_inspect_npc(
    inspect_npc: &InspectNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();
    let npc_id = parse_id(&inspect_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Box::new(NpcNotFoundError(npc_id.to_string()))),
    };

    let mut rng = rand::thread_rng();

    if inspect_npc.discover_health && roll_d6(&mut rng, 1, 0) >= DISCOVER_HEALTH_CHANCE {
        events.push(Event::NpcHealthDiscovered(NpcHealthDiscovered { npc_id }));
    }

    if inspect_npc.discover_name && roll_d6(&mut rng, 1, 0) >= DISCOVER_NAME_CHANCE {
        events.push(Event::NpcNameDiscovered(NpcNameDiscovered { npc_id }));
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

    Ok(events)
}
