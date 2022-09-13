use crate::components::{
    CharacterViewArgs, {PlayerCharacter, PlayerCharacterView},
};

pub fn check(player_character: &PlayerCharacter) -> PlayerCharacterView {
    let args = CharacterViewArgs {
        knows_health: true,
        knows_inventory: true,
        knows_packed_in_inventory: true,
    };
    let character = super::character::view(&player_character.character, &args, true);

    PlayerCharacterView {
        character,
        id: player_character.id.to_string(),
        gold: player_character.gold,
        name: player_character.name.clone(),
    }
}
