use crate::{
    actions::InspectNpc,
    components::games::GameState,
    errors::Error,
    events::{Event, NpcHealthDiscovered, NpcPackedDiscovered},
    utils::{ids::parse_id, rolls::roll_d6},
};

const DISCOVER_HEALTH_CHANCE: i32 = 5;
const DISCOVER_PACKED_CHANCE: i32 = 4;

pub fn handle(inspect_npc: &InspectNpc, state: &GameState) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = Vec::new();
    let npc_id = parse_id(&inspect_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    if npc.character.is_dead() {
        events.push(Event::NpcHealthDiscovered(NpcHealthDiscovered { npc_id }));
        events.push(Event::NpcPackedDiscovered(NpcPackedDiscovered { npc_id }));
    } else {
        let mut rng = rand::thread_rng();

        if inspect_npc.discover_health && roll_d6(&mut rng, 1, 0) >= DISCOVER_HEALTH_CHANCE {
            events.push(Event::NpcHealthDiscovered(NpcHealthDiscovered { npc_id }));
        }

        if inspect_npc.discover_packed_items && roll_d6(&mut rng, 1, 0) >= DISCOVER_PACKED_CHANCE {
            events.push(Event::NpcPackedDiscovered(NpcPackedDiscovered { npc_id }));
        }
    }

    Ok(events)
}
