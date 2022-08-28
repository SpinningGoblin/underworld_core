#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Character, CharacterView, CharacterViewArgs};

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
        self.character.kill();
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "NonPlayer"))]
pub struct NonPlayerView {
    pub character: CharacterView,
    pub can_be_looted: bool,
    pub id: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct NonPlayerViewArgs {
    pub character_args: CharacterViewArgs,
}

impl NonPlayerViewArgs {
    pub fn knows_all_args() -> NonPlayerViewArgs {
        NonPlayerViewArgs {
            character_args: CharacterViewArgs::knows_all_args(),
        }
    }
}
