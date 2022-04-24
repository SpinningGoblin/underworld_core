use crate::{
    actions::look_at::LookAtNpc,
    components::games::game_state::GameState,
    errors::Errors,
    events::{event::Event, npc_viewed::NpcViewed},
    systems::view::non_player,
    utils::ids::parse_id,
};

pub fn handle_view_npc(game_state: &GameState, action: &LookAtNpc) -> Result<Vec<Event>, Errors> {
    let npc_id = parse_id(&action.npc_id)?;

    let npc = match game_state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Errors::NpcNotFound(npc_id.to_string())),
    };

    let view = non_player::look_at(npc, &action.args, action.knows_name, action.knows_all);

    Ok(vec![Event::NpcViewed(NpcViewed { npc_view: view })])
}
