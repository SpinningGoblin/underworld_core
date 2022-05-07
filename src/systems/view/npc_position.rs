use crate::components::{
    non_player::{NonPlayerView, NonPlayerViewArgs},
    rooms::npc_position::{NpcPosition, NpcPositionView},
};

pub fn look_at(
    npc_position: &NpcPosition,
    npc_position_args: &NonPlayerViewArgs,
    knows_all: bool,
) -> NpcPositionView {
    let npcs: Vec<NonPlayerView> = npc_position
        .npcs
        .iter()
        .map(|non_player| {
            super::non_player::look_at(
                non_player,
                &npc_position_args.character_args,
                npc_position_args.knows_name,
                knows_all,
            )
        })
        .into_iter()
        .collect();

    NpcPositionView {
        group_descriptor: npc_position.group_descriptor.clone(),
        npcs,
        position_descriptor: npc_position.position_descriptor.clone(),
    }
}
