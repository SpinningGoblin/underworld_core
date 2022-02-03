use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::object_tag::{ObjectTag, TaggedObject};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
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

impl TaggedObject for Material {
    fn tags(&self) -> Vec<ObjectTag> {
        match *self {
            Material::Bone => vec![ObjectTag::Bone],
            Material::Gold => vec![ObjectTag::Metal],
            Material::Hide => vec![ObjectTag::Leather],
            Material::Iron => vec![ObjectTag::Metal],
            Material::Leather => vec![ObjectTag::Leather],
            Material::Steel => vec![ObjectTag::Metal],
            Material::Stone => vec![ObjectTag::Stone],
            Material::Wooden => vec![ObjectTag::Wood],
            Material::Cotton => vec![ObjectTag::Cloth],
            Material::Fur => vec![ObjectTag::Cloth],
            Material::Linen => vec![ObjectTag::Cloth],
            Material::Silk => vec![ObjectTag::Cloth],
            Material::Wool => vec![ObjectTag::Cloth],
        }
    }
}
