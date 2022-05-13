use crate::{
    actions::look_at_npc::LookAtNpc,
    components::{character::CharacterViewArgs, games::game_state::GameState},
    errors::Errors,
    events::{event::Event, npc_viewed::NpcViewed},
    systems::view::non_player,
    utils::ids::parse_id,
};

pub fn handle_view_npc(look_at_npc: &LookAtNpc, state: &GameState) -> Result<Vec<Event>, Errors> {
    let npc_id = parse_id(&look_at_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Errors::NpcNotFound(npc_id.to_string())),
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

    let view = non_player::look_at(npc, &args, knowledge.knows_name, state.player_knows_all);

    Ok(vec![Event::NpcViewed(NpcViewed { npc_view: view })])
}
