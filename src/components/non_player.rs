#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{character::Character, identifier::Identifier};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct NonPlayer {
    pub character: Character,
    pub identifier: Identifier,
}

impl NonPlayer {
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
