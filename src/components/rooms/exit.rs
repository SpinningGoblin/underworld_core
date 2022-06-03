#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{material::Material, size::Size};

use super::{exit_descriptor::ExitDescriptor, exit_type::ExitType};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Exit {
    pub id: Uuid,
    pub name: Option<String>,
    pub exit_type: ExitType,
    pub material: Option<Material>,
    pub descriptors: Vec<ExitDescriptor>,
    pub size: Option<Size>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Exit"))]
pub struct ExitView {
    pub id: String,
    pub name: Option<String>,
    pub exit_type: ExitType,
    pub material: Option<Material>,
    pub descriptors: Vec<ExitDescriptor>,
    pub size: Option<Size>,
}
