use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use poem_openapi::Enum;

use super::tag::{Tag, Tagged};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all ="snake_case"))]
pub enum Material {
    Bone,
    Cotton,
    Fur,
    Gold,
    Hide,
    Iron,
    Leather,
    Linen,
    Silk,
    Steel,
    Stone,
    Wooden,
    Wool,
}

pub trait BuiltWithMaterial {
    fn possible_materials(&self) -> Vec<Material>;
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Material::Bone => write!(f, "bone"),
            Material::Gold => write!(f, "gold"),
            Material::Hide => write!(f, "hide"),
            Material::Iron => write!(f, "iron"),
            Material::Leather => write!(f, "leather"),
            Material::Steel => write!(f, "steel"),
            Material::Stone => write!(f, "stone"),
            Material::Wooden => write!(f, "wooden"),
            Material::Cotton => write!(f, "cotton"),
            Material::Fur => write!(f, "fur"),
            Material::Linen => write!(f, "linen"),
            Material::Silk => write!(f, "silk"),
            Material::Wool => write!(f, "wool"),
        }
    }
}

impl Tagged for Material {
    fn tags(&self) -> Vec<Tag> {
        match *self {
            Material::Bone => vec![Tag::Bone],
            Material::Gold => vec![Tag::Metal],
            Material::Hide => vec![Tag::Leather],
            Material::Iron => vec![Tag::Metal],
            Material::Leather => vec![Tag::Leather],
            Material::Steel => vec![Tag::Metal],
            Material::Stone => vec![Tag::Stone],
            Material::Wooden => vec![Tag::Wood],
            Material::Cotton => vec![Tag::Cloth],
            Material::Fur => vec![Tag::Cloth],
            Material::Linen => vec![Tag::Cloth],
            Material::Silk => vec![Tag::Cloth],
            Material::Wool => vec![Tag::Cloth],
        }
    }
}
