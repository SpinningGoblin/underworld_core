use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    rooms::{NpcPosition, NpcPositionView},
    NonPlayerViewArgs,
};

pub fn view(
    npc_position: &NpcPosition,
    non_player_args: &HashMap<Uuid, NonPlayerViewArgs>,
    knows_all: bool,
) -> NpcPositionView {
    let args = non_player_args
        .get(&npc_position.npc.id)
        .cloned()
        .unwrap_or_else(|| {
            if knows_all {
                NonPlayerViewArgs::knows_all_args()
            } else {
                NonPlayerViewArgs::default()
            }
        });
    let npc = super::non_player::view(&npc_position.npc, &args.character_args, knows_all);
    NpcPositionView {
        npc,
        position_descriptor: npc_position.position_descriptor,
    }
}
