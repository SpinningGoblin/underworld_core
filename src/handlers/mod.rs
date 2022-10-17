mod attack_npc;
mod cast_spell_on_npc;
mod cast_spell_on_player;
mod exit_room;
mod global_effects;
mod handle;
mod handle_npc_action;
mod helpers;
mod inspect_fixture;
mod inspect_npc;
mod loot_fixture;
mod loot_npc;
mod move_player_item;
mod npc_action;
mod open_fixture;
mod open_fixture_hidden_compartment;
mod pick_up_item;
mod sell_player_item;
mod throw_item_at_npc;
mod use_item_on_player;
mod view_fixture;
mod view_npc;

use handle_npc_action::handle_npc_action;
use npc_action::NpcAction;

pub use handle::{handle_action, HandledAction};
