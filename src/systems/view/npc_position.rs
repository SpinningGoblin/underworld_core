use crate::components::{
    non_player::NonPlayerView,
    rooms::npc_position::{NpcPosition, NpcPositionView, NpcPositionViewArgs},
};

pub fn look_at(
    npc_position: &NpcPosition,
    npc_position_args: &NpcPositionViewArgs,
    knows_all: bool,
) -> NpcPositionView {
    let npcs: Vec<NonPlayerView> = npc_position
        .npcs
        .iter()
        .map(|npc| {
            super::non_player::look_at(
                &npc,
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
