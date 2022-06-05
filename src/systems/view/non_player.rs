use crate::components::{
    character::CharacterViewArgs,
    non_player::{NonPlayer, NonPlayerView},
};

pub fn view(
    non_player: &NonPlayer,
    character_args: &CharacterViewArgs,
    knows_all: bool,
) -> NonPlayerView {
    NonPlayerView {
        id: non_player.id.to_string(),
        name: non_player.name.clone(),
        character: super::character::view(&non_player.character, character_args, knows_all),
    }
}
