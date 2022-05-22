use crate::components::{
    character::CharacterViewArgs,
    player::{PlayerCharacter, PlayerCharacterView},
};

pub fn check(player_character: PlayerCharacter) -> PlayerCharacterView {
    let args = CharacterViewArgs {
        knows_health: true,
        knows_species: true,
        knows_life_modifier: true,
        knows_inventory: true,
        knows_hidden_in_inventory: true,
        knows_packed_in_inventory: true,
    };
    let character = super::character::view(&player_character.character, &args, true);

    PlayerCharacterView {
        character,
        identifier: super::identifier::view(&player_character.identifier, true),
    }
}
