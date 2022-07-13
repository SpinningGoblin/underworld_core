use crate::{
    components::{games::GameState, PlayerCharacter},
    errors::Error,
    events::Event,
};

use super::{helpers::npc_attack_player, NpcAction};

pub fn handle_npc_action(
    npc_action: &NpcAction,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    match npc_action {
        NpcAction::AttackPlayer(npc_id) => {
            let npc = match state.current_room().find_npc(npc_id) {
                Some(it) => it,
                None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
            };

            Ok(npc_attack_player(player, npc, true))
        }
    }
}
