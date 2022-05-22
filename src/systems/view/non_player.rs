use crate::components::{
    character::CharacterViewArgs,
    non_player::{NonPlayer, NonPlayerView},
};

pub fn view(
    non_player: &NonPlayer,
    character_args: &CharacterViewArgs,
    knows_name: bool,
    knows_all: bool,
) -> NonPlayerView {
    NonPlayerView {
        identifier: super::identifier::view(&non_player.identifier, knows_name || knows_all),
        character: super::character::view(&non_player.character, character_args, knows_all),
    }
}
