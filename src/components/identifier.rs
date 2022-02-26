#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use uuid::Uuid;

use std::fmt::Display;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Identifier {
    pub id: Uuid,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct IdentifierView {
    pub id: String,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub name: Option<String>,
    pub name_known: bool,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)
        } else {
            write!(f, "")
        }
    }
}

impl Identifier {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}
