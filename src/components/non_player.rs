#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    character::{Character, CharacterView, CharacterViewArgs},
    identifier::{Identifier, IdentifierView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct NonPlayer {
    pub character: Character,
    pub identifier: Identifier,
}

pub struct NonPlayerView {
    pub character: CharacterView,
    pub identifier: IdentifierView,
}

impl NonPlayer {
    pub fn look_at(
        &self,
        character_args: CharacterViewArgs,
        knows_name: bool,
        knows_all: bool,
    ) -> NonPlayerView {
        let identifier = if knows_name || knows_all {
            IdentifierView {
                id: self.identifier.id,
                name: self.identifier.name.clone(),
                name_known: true,
            }
        } else {
            IdentifierView {
                id: self.identifier.id,
                name: None,
                name_known: false,
            }
        };

        NonPlayerView {
            identifier,
            character: self.character.look_at(character_args, knows_all),
        }
    }

    pub fn describe_name(&self) -> String {
        match &self.identifier.name {
            Some(name) => format!("It says its name is {}", name),
            _ => "It has no name.".to_string(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.identifier.set_name(name);
    }

    pub fn describe(&self, starter: &str) -> String {
        let descriptions: Vec<String> = vec![self.character.describe_inventory(starter)];

        descriptions.join("")
    }
}
