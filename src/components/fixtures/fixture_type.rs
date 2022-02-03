#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    material::{BuiltWithMaterial, Material},
    object_tag::{ObjectTag, TaggedObject},
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

impl TaggedObject for FixtureType {
    fn tags(&self) -> Vec<ObjectTag> {
        match *self {
            FixtureType::Bed => vec![ObjectTag::Cloth, ObjectTag::Fixture],
            FixtureType::Chair => vec![ObjectTag::Fixture],
            FixtureType::Chest => vec![ObjectTag::Fixture, ObjectTag::Container],
            FixtureType::Cot => vec![ObjectTag::Fixture, ObjectTag::Cloth],
            FixtureType::SleepingRoll => {
                vec![ObjectTag::Fixture, ObjectTag::Cloth, ObjectTag::Leather]
            }
            FixtureType::Table => vec![ObjectTag::Fixture],
            FixtureType::WeaponRack => vec![ObjectTag::Fixture],
            FixtureType::Barrel => vec![ObjectTag::Fixture, ObjectTag::Container],
            FixtureType::Crate => vec![ObjectTag::Fixture, ObjectTag::Container],
        }
    }
}

impl BuiltWithMaterial for FixtureType {
    fn possible_materials(&self) -> Vec<Material> {
        match *self {
            FixtureType::Barrel => vec![Material::Wooden],
            FixtureType::Bed => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::Chair => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::Chest => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::Cot => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::Crate => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::SleepingRoll => vec![
                Material::Wool,
                Material::Linen,
                Material::Hide,
                Material::Leather,
                Material::Cotton,
            ],
            FixtureType::Table => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::WeaponRack => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
            ],
        }
    }
}
