#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::{Tag, Tagged};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum Material {
    Bone,
    Ceramic,
    Cotton,
    Fur,
    Glass,
    Gold,
    Hide,
    Iron,
    Leather,
    Linen,
    Paper,
    Papyrus,
    Silk,
    Steel,
    Stone,
    Wooden,
    Wool,
}

pub trait BuiltWithMaterial {
    fn possible_materials(&self) -> Vec<Material>;
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
            Material::Paper => vec![Tag::Paper],
            Material::Papyrus => vec![Tag::Paper],
            Material::Ceramic => vec![Tag::Ceramic],
            Material::Glass => vec![Tag::Container],
        }
    }
}
