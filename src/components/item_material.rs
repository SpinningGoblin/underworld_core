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
    fn tag(&self) -> ItemTag {
        match *self {
            ItemMaterial::Bone => ItemTag::Bone,
            ItemMaterial::Gold => ItemTag::Metal,
            ItemMaterial::Hide => ItemTag::Leather,
            ItemMaterial::Iron => ItemTag::Metal,
            ItemMaterial::Leather => ItemTag::Leather,
            ItemMaterial::Steel => ItemTag::Metal,
            ItemMaterial::Stone => ItemTag::Stone,
            ItemMaterial::Wooden => ItemTag::Wood,
            ItemMaterial::Cotton => ItemTag::Cloth,
            ItemMaterial::Fur => ItemTag::Cloth,
            ItemMaterial::Linen => ItemTag::Cloth,
            ItemMaterial::Silk => ItemTag::Cloth,
            ItemMaterial::Wool => ItemTag::Cloth,
        }
    }
}
