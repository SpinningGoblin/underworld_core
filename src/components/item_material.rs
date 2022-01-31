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
pub enum ItemMaterial {
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
    fn possible_materials(&self) -> Vec<ItemMaterial>;
}

impl Display for ItemMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ItemMaterial::Bone => write!(f, "bone"),
            ItemMaterial::Gold => write!(f, "gold"),
            ItemMaterial::Hide => write!(f, "hide"),
            ItemMaterial::Iron => write!(f, "iron"),
            ItemMaterial::Leather => write!(f, "leather"),
            ItemMaterial::Steel => write!(f, "steel"),
            ItemMaterial::Stone => write!(f, "stone"),
            ItemMaterial::Wooden => write!(f, "wooden"),
            ItemMaterial::Cotton => write!(f, "cotton"),
            ItemMaterial::Fur => write!(f, "fur"),
            ItemMaterial::Linen => write!(f, "linen"),
            ItemMaterial::Silk => write!(f, "silk"),
            ItemMaterial::Wool => write!(f, "wool"),
        }
    }
}

impl TaggedItem for ItemMaterial {
    fn tags(&self) -> Vec<ItemTag> {
        match *self {
            ItemMaterial::Bone => vec![ItemTag::Bone],
            ItemMaterial::Gold => vec![ItemTag::Metal],
            ItemMaterial::Hide => vec![ItemTag::Leather],
            ItemMaterial::Iron => vec![ItemTag::Metal],
            ItemMaterial::Leather => vec![ItemTag::Leather],
            ItemMaterial::Steel => vec![ItemTag::Metal],
            ItemMaterial::Stone => vec![ItemTag::Stone],
            ItemMaterial::Wooden => vec![ItemTag::Wood],
            ItemMaterial::Cotton => vec![ItemTag::Cloth],
            ItemMaterial::Fur => vec![ItemTag::Cloth],
            ItemMaterial::Linen => vec![ItemTag::Cloth],
            ItemMaterial::Silk => vec![ItemTag::Cloth],
            ItemMaterial::Wool => vec![ItemTag::Cloth],
        }
    }
}
