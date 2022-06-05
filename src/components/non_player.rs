#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::character::{Character, CharacterView, CharacterViewArgs};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct NonPlayer {
    pub character: Character,
    pub id: Uuid,
    pub name: Option<String>,
}

impl NonPlayer {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn kill(&mut self) {
        if let Some(current) = self.character.get_current_health() {
            self.character.damage(current)
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "NonPlayer"))]
pub struct NonPlayerView {
    pub character: CharacterView,
    pub id: String,
    pub name: Option<String>,
}

impl NonPlayerView {
    pub fn describe_name(&self) -> String {
        match &self.name {
            Some(name) => format!("It says its name is {}", name),
            _ => "It has no name.".to_string(),
        }
    }

    pub fn describe(&self, starter: &str) -> String {
        let descriptions: Vec<String> = vec![self.character.describe_inventory(starter)];

        descriptions.join("")
    }
}

#[derive(Clone, Debug, Default)]
pub struct NonPlayerViewArgs {
    pub character_args: CharacterViewArgs,
}
