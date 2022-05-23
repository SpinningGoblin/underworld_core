use std::error::Error;

use crate::{
    actions::look_at_npc::LookAtNpc,
    components::{character::CharacterViewArgs, games::game_state::GameState},
    errors::npc_not_found_error::NpcNotFoundError,
    events::{Event, NpcViewed},
    systems::view::non_player,
    utils::ids::parse_id,
};

pub fn handle(look_at_npc: &LookAtNpc, state: &GameState) -> Result<Vec<Event>, Box<dyn Error>> {
    let npc_id = parse_id(&look_at_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Box::new(NpcNotFoundError(npc_id.to_string()))),
    };

    let knowledge = state.npc_knowledge(&npc_id);
    let args = CharacterViewArgs {
        knows_health: knowledge.knows_health,
        knows_species: knowledge.knows_species,
        knows_life_modifier: knowledge.knows_life_modifier,
        knows_inventory: knowledge.knows_inventory,
        knows_hidden_in_inventory: knowledge.knows_hidden_in_inventory,
        knows_packed_in_inventory: knowledge.knows_packed_in_inventory,
    };

    let view = non_player::view(npc, &args, knowledge.knows_name, state.player_knows_all);

    Ok(vec![Event::NpcViewed(NpcViewed { npc_view: view })])
}
