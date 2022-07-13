#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Character, CharacterView};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct PlayerCharacter {
    pub character: Character,
    pub id: Uuid,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub gold: u32,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "PlayerCharacter"))]
pub struct PlayerCharacterView {
    pub character: CharacterView,
    pub id: String,
    pub gold: u32,
    pub name: Option<String>,
}
