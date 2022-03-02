#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fmt::Display;

#[derive(Clone, Debug, Default)]
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

    pub fn to_view(&self, name_known: bool) -> IdentifierView {
        if name_known {
            IdentifierView {
                id: self.id.to_string(),
                name: self.name.clone(),
                name_known,
            }
        } else {
            IdentifierView {
                id: self.id.to_string(),
                name: None,
                name_known,
            }
        }
    }
}
