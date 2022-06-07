use crate::{
    actions::LookAtNpc,
    components::{character::CharacterViewArgs, games::game_state::GameState},
    errors::Error,
    events::{Event, NpcViewed},
    systems::view::non_player,
    utils::ids::parse_id,
};

pub fn handle(look_at_npc: &LookAtNpc, state: &GameState) -> Result<Vec<Event>, Error> {
    let npc_id = parse_id(&look_at_npc.npc_id)?;

    let npc = match state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => {
            return Err(Error::NpcNotFoundError(npc_id.to_string()))
        }
    };

    let knowledge = state.npc_knowledge(&npc_id);
    let args = CharacterViewArgs {
        knows_health: knowledge.knows_health,
        knows_inventory: knowledge.knows_inventory,
        knows_hidden_in_inventory: knowledge.knows_hidden_in_inventory,
        knows_packed_in_inventory: knowledge.knows_packed_in_inventory,
    };

    let view = non_player::view(npc, &args, state.player_knows_all);

    Ok(vec![Event::NpcViewed(NpcViewed { npc_view: view })])
}
