use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::item_tag::{ItemTag, TaggedItem};

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

impl TaggedItem for Material {
    fn tags(&self) -> Vec<ItemTag> {
        match *self {
            Material::Bone => vec![ItemTag::Bone],
            Material::Gold => vec![ItemTag::Metal],
            Material::Hide => vec![ItemTag::Leather],
            Material::Iron => vec![ItemTag::Metal],
            Material::Leather => vec![ItemTag::Leather],
            Material::Steel => vec![ItemTag::Metal],
            Material::Stone => vec![ItemTag::Stone],
            Material::Wooden => vec![ItemTag::Wood],
            Material::Cotton => vec![ItemTag::Cloth],
            Material::Fur => vec![ItemTag::Cloth],
            Material::Linen => vec![ItemTag::Cloth],
            Material::Silk => vec![ItemTag::Cloth],
            Material::Wool => vec![ItemTag::Cloth],
        }
    }
}
