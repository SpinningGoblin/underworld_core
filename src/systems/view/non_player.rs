use crate::components::{
    character::CharacterViewArgs,
    non_player::{NonPlayer, NonPlayerView},
};

pub fn look_at(
    non_player: &NonPlayer,
    character_args: &CharacterViewArgs,
    knows_name: bool,
    knows_all: bool,
) -> NonPlayerView {
    NonPlayerView {
        identifier: super::identifier::to_view(&non_player.identifier, knows_name || knows_all),
        character: super::character::look_at(&non_player.character, character_args, knows_all),
    }
}
