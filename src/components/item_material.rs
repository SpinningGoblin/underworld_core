use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    descriptor_tags::{DescriptorTag, DescriptorTagged},
    item_descriptor::ItemDescriptor,
};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum ItemMaterial {
    Bone,
    Cloth,
    Gold,
    Hide,
    Iron,
    Leather,
    Steel,
    Stone,
    Wooden,
}

pub trait BuiltWithMaterial {
    fn possible_materials(&self) -> Vec<ItemMaterial>;
}

impl Display for ItemMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ItemMaterial::Bone => write!(f, "bone"),
            ItemMaterial::Cloth => write!(f, "cloth"),
            ItemMaterial::Gold => write!(f, "gold"),
            ItemMaterial::Hide => write!(f, "hide"),
            ItemMaterial::Iron => write!(f, "iron"),
            ItemMaterial::Leather => write!(f, "leather"),
            ItemMaterial::Steel => write!(f, "steel"),
            ItemMaterial::Stone => write!(f, "stone"),
            ItemMaterial::Wooden => write!(f, "wooden"),
        }
    }
}

impl DescriptorTagged for ItemMaterial {
    fn descriptor_tag(&self) -> DescriptorTag {
        match *self {
            ItemMaterial::Bone => DescriptorTag::Bone,
            ItemMaterial::Cloth => DescriptorTag::Cloth,
            ItemMaterial::Gold => DescriptorTag::Metal,
            ItemMaterial::Hide => DescriptorTag::Leather,
            ItemMaterial::Iron => DescriptorTag::Metal,
            ItemMaterial::Leather => DescriptorTag::Leather,
            ItemMaterial::Steel => DescriptorTag::Metal,
            ItemMaterial::Stone => DescriptorTag::Stone,
            ItemMaterial::Wooden => DescriptorTag::Wood,
        }
    }
}

impl ItemMaterial {
    pub fn possible_descriptors(&self) -> Vec<ItemDescriptor> {
        match *self {
            ItemMaterial::Bone => ItemMaterial::bone_descriptors(),
            ItemMaterial::Cloth => ItemMaterial::cloth_descriptors(),
            ItemMaterial::Gold => ItemMaterial::metal_descriptors(),
            ItemMaterial::Hide => ItemMaterial::leather_descriptors(),
            ItemMaterial::Iron => ItemMaterial::metal_descriptors(),
            ItemMaterial::Leather => ItemMaterial::leather_descriptors(),
            ItemMaterial::Steel => ItemMaterial::metal_descriptors(),
            ItemMaterial::Stone => ItemMaterial::stone_descriptors(),
            ItemMaterial::Wooden => ItemMaterial::wood_descriptors(),
        }
    }

    pub fn bone_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Bleached,
            ItemDescriptor::Cracked,
            ItemDescriptor::Scuffed,
            ItemDescriptor::Smoothed,
            ItemDescriptor::Weathered,
        ]
    }

    pub fn cloth_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Bleached,
            ItemDescriptor::Dingy,
            ItemDescriptor::Drab,
            ItemDescriptor::Ripped,
            ItemDescriptor::Stained,
            ItemDescriptor::Torn,
            ItemDescriptor::WaterLogged,
            ItemDescriptor::Weathered,
        ]
    }

    pub fn leather_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Beaten,
            ItemDescriptor::Cracked,
            ItemDescriptor::Crumbling,
            ItemDescriptor::Scuffed,
            ItemDescriptor::Stained,
            ItemDescriptor::WaterLogged,
            ItemDescriptor::Weathered,
        ]
    }

    pub fn metal_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Chipped,
            ItemDescriptor::Rusty,
            ItemDescriptor::Scuffed,
            ItemDescriptor::Shiny,
            ItemDescriptor::Tarnished,
            ItemDescriptor::Weathered,
        ]
    }

    pub fn stone_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Chipped,
            ItemDescriptor::Cracked,
            ItemDescriptor::Smoothed,
            ItemDescriptor::Weathered,
        ]
    }

    pub fn wood_descriptors() -> Vec<ItemDescriptor> {
        vec![
            ItemDescriptor::Bleached,
            ItemDescriptor::Cracked,
            ItemDescriptor::Splintered,
            ItemDescriptor::Weathered,
        ]
    }
}
