use super::{
    attack_npc::AttackNpc,
    exit_room::ExitRoom,
    look_at::{LookAtCurrentRoom, LookAtNpc},
    loot_npc::LootNpc,
    move_player_item::MovePlayerItem,
    quick_look::QuickLookCurrentRoom,
};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Action {
    AttackNpc(AttackNpc),
    LookAtCurrentRoom(LookAtCurrentRoom),
    LookAtNpc(LookAtNpc),
    LootNpc(LootNpc),
    MovePlayerItem(MovePlayerItem),
    QuickLookCurrentRoom(QuickLookCurrentRoom),
    ExitRoom(ExitRoom),
}
