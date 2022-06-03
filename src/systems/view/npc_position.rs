use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    non_player::NonPlayerViewArgs,
    rooms::npc_position::{NpcPosition, NpcPositionView},
};

pub fn view(
    npc_position: &NpcPosition,
    non_player_args: &HashMap<Uuid, NonPlayerViewArgs>,
    knows_all: bool,
) -> NpcPositionView {
    let args = non_player_args
        .get(&npc_position.npc.id)
        .cloned()
        .unwrap_or_default();
    let npc = super::non_player::view(
        &npc_position.npc,
        &args.character_args,
        args.knows_name,
        knows_all,
    );
    NpcPositionView {
        group_descriptor: npc_position.group_descriptor.clone(),
        npc,
        position_descriptor: npc_position.position_descriptor.clone(),
    }
}
