use crate::components::{
    CharacterViewArgs, {NonPlayer, NonPlayerView},
};

pub fn view(
    non_player: &NonPlayer,
    character_args: &CharacterViewArgs,
    knows_all: bool,
) -> NonPlayerView {
    let can_be_looted =
        (character_args.knows_health || knows_all) && non_player.character.is_dead();

    NonPlayerView {
        id: non_player.id.to_string(),
        can_be_looted,
        name: non_player.name.clone(),
        character: super::character::view(&non_player.character, character_args, knows_all),
    }
}
