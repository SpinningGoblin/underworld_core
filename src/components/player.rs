#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    character::{Character, CharacterView, CharacterViewArgs},
    identifier::{Identifier, IdentifierView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct PlayerCharacter {
    pub character: Character,
    pub identifier: Identifier,
    pub username: String,
}

impl PlayerCharacter {
    pub fn check(&self) -> PlayerCharacterView {
        let args = CharacterViewArgs {
            knows_health: true,
            knows_species: true,
            knows_life_modifier: true,
            knows_inventory: true,
            knows_hidden_in_inventory: true,
            knows_packed_in_inventory: true,
        };
        let character = self.character.look_at(&args, true);
        let identifier = self.identifier.to_view(true);

        PlayerCharacterView {
            character,
            identifier,
            username: self.username.clone(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct PlayerCharacterView {
    pub character: CharacterView,
    pub identifier: IdentifierView,
    pub username: String,
}
