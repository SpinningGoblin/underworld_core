#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    item_material::{BuiltWithMaterial, ItemMaterial},
    item_tag::{ItemTag, TaggedItem},
};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum FixtureType {
    Barrel,
    Bed,
    Chair,
    Chest,
    Cot,
    Crate,
    SleepingRoll,
    Table,
    WeaponRack,
}

impl Display for FixtureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FixtureType::Table => write!(f, "table"),
            FixtureType::Chair => write!(f, "chair"),
            FixtureType::Chest => write!(f, "chest"),
            FixtureType::WeaponRack => write!(f, "weapon rack"),
            FixtureType::Bed => write!(f, "bed"),
            FixtureType::Cot => write!(f, "cot"),
            FixtureType::SleepingRoll => write!(f, "sleeping roll"),
            FixtureType::Barrel => write!(f, "barrel"),
            FixtureType::Crate => write!(f, "crate"),
        }
    }
}

impl TaggedItem for FixtureType {
    fn tags(&self) -> Vec<ItemTag> {
        match *self {
            FixtureType::Bed => vec![ItemTag::Cloth, ItemTag::Fixture],
            FixtureType::Chair => vec![ItemTag::Fixture],
            FixtureType::Chest => vec![ItemTag::Fixture, ItemTag::Container],
            FixtureType::Cot => vec![ItemTag::Fixture, ItemTag::Cloth],
            FixtureType::SleepingRoll => vec![ItemTag::Fixture, ItemTag::Cloth, ItemTag::Leather],
            FixtureType::Table => vec![ItemTag::Fixture],
            FixtureType::WeaponRack => vec![ItemTag::Fixture],
            FixtureType::Barrel => vec![ItemTag::Fixture, ItemTag::Container],
            FixtureType::Crate => vec![ItemTag::Fixture, ItemTag::Container],
        }
    }
}

impl BuiltWithMaterial for FixtureType {
    fn possible_materials(&self) -> Vec<ItemMaterial> {
        match *self {
            FixtureType::Barrel => vec![ItemMaterial::Wooden],
            FixtureType::Bed => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::Chair => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::Chest => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::Cot => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::Crate => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::SleepingRoll => vec![
                ItemMaterial::Wool,
                ItemMaterial::Linen,
                ItemMaterial::Hide,
                ItemMaterial::Leather,
                ItemMaterial::Cotton,
            ],
            FixtureType::Table => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            FixtureType::WeaponRack => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
        }
    }
}
