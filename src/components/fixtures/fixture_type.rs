#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::components::{
    {BuiltWithMaterial, Material}, {Tag, Tagged},
};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum FixtureType {
    Barrel,
    Bed,
    Bucket,
    Chair,
    Chest,
    Coffin,
    Cot,
    Crate,
    Pillar,
    SleepingRoll,
    StatueTentacledMonstrosity,
    StatueWarrior,
    Table,
    WeaponRack,
}

impl Tagged for FixtureType {
    fn tags(&self) -> Vec<Tag> {
        match *self {
            FixtureType::Bed => vec![Tag::Cloth, Tag::Fixture],
            FixtureType::Chair => vec![Tag::Fixture],
            FixtureType::Chest => vec![Tag::Fixture, Tag::Container],
            FixtureType::Cot => vec![Tag::Fixture, Tag::Cloth],
            FixtureType::SleepingRoll => {
                vec![Tag::Fixture, Tag::Cloth, Tag::Leather]
            }
            FixtureType::Table => vec![Tag::Fixture],
            FixtureType::WeaponRack => vec![Tag::Fixture],
            FixtureType::Barrel => vec![Tag::Fixture, Tag::Container],
            FixtureType::Crate => vec![Tag::Fixture, Tag::Container],
            FixtureType::Bucket => vec![Tag::Fixture, Tag::Container],
            FixtureType::Coffin => vec![Tag::Fixture, Tag::Container],
            FixtureType::Pillar
            | FixtureType::StatueTentacledMonstrosity
            | FixtureType::StatueWarrior => vec![Tag::Fixture],
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
            FixtureType::Bucket => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Stone,
                Material::Wooden,
            ],
            FixtureType::Coffin => vec![
                Material::Bone,
                Material::Gold,
                Material::Wooden,
                Material::Stone,
            ],
            FixtureType::Pillar => vec![Material::Bone, Material::Gold, Material::Stone],
            FixtureType::StatueTentacledMonstrosity => vec![Material::Bone, Material::Stone],
            FixtureType::StatueWarrior => vec![
                Material::Gold,
                Material::Stone,
                Material::Steel,
                Material::Iron,
            ],
        }
    }
}
