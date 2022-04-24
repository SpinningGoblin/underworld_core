use crate::{
    actions::look_at::LookAtNpc,
    components::games::game_state::GameState,
    events::{event::Event, npc_viewed::NpcViewed},
    systems::view::non_player,
    utils::ids::parse_id,
};

pub fn handle_view_npc(game_state: &GameState, action: &LookAtNpc) -> Vec<Event> {
    let npc_id = match parse_id(&action.npc_id) {
        Some(it) => it,
        None => return Vec::new(),
    };

    let npc = match game_state.current_room().find_npc(&npc_id) {
        Some(it) => it,
        None => return Vec::new(),
    };

    let view = non_player::look_at(npc, &action.args, action.knows_name, action.knows_all);

    vec![Event::NpcViewed(NpcViewed { npc_view: view })]
}
