use crate::{
    actions::loot_npc::LootNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Errors,
    events::{event::Event, item_taken_from_npc::ItemTakenFromNpc},
    utils::ids::parse_id,
};

use super::helpers::npc_attack_player;

pub fn handle_loot_npc(
    state: &GameState,
    action: &LootNpc,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Errors> {
    let npc_id = parse_id(&action.npc_id)?;

    let room = state.current_room();

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Errors::NpcNotFound(npc_id.to_string())),
    };

    let mut events: Vec<Event> = Vec::new();

    if !npc.character.is_dead() {
        events.append(&mut npc_attack_player(player, npc));
    } else {
        for id in &action.item_ids {
            let item_id = parse_id(id)?;
            match npc.character.find_item(&item_id) {
                Some(_) => events.push(Event::ItemTakenFromNpc(ItemTakenFromNpc {
                    item_id,
                    npc_id: npc_id.clone(),
                })),
                None => return Err(Errors::ItemNotFound(item_id.to_string())),
            }
        }
    }

    Ok(events)
}
