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
    let name = if knows_name || knows_all {
        non_player.name.clone()
    } else {
        None
    };

    NonPlayerView {
        id: non_player.id.to_string(),
        name,
        character: super::character::view(&non_player.character, character_args, knows_all),
    }
}
