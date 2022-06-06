use crate::{
    actions::LootNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::{Error, ItemNotFoundError, NpcNotFoundError},
    events::{Event, ItemTakenFromNpc},
    utils::ids::parse_id,
};

use super::helpers::npc_attack_player;

pub fn handle(
    loot_npc: &LootNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let npc_id = parse_id(&loot_npc.npc_id)?;

    let room = state.current_room();

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => {
            return Err(Error::NpcNotFoundError(NpcNotFoundError(
                npc_id.to_string(),
            )))
        }
    };

    let mut events: Vec<Event> = Vec::new();

    if !npc.character.is_dead() {
        events.append(&mut npc_attack_player(player, npc));
    } else {
        for id in &loot_npc.item_ids {
            let item_id = parse_id(id)?;
            match npc.character.find_item(&item_id) {
                Some(_) => events.push(Event::ItemTakenFromNpc(ItemTakenFromNpc {
                    item_id,
                    npc_id,
                })),
                None => {
                    return Err(Error::ItemNotFoundError(ItemNotFoundError(
                        item_id.to_string(),
                    )))
                }
            }
        }
    }

    Ok(events)
}
