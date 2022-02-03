use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    equipment::{location_tag::LocationTag, Equipment},
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
pub enum WeaponType {
    Buckler,
    Club,
    Dagger,
    Dirk,
    GreatSword,
    Hammer,
    LongSword,
    Mace,
    Morningstar,
    Shield,
    ShortSword,
    Whip,
}

impl BuiltWithMaterial for WeaponType {
    fn possible_materials(&self) -> Vec<Material> {
        match *self {
            WeaponType::Buckler => {
                vec![Material::Hide, Material::Iron, Material::Steel]
            }
            WeaponType::Club => vec![Material::Bone, Material::Stone, Material::Wooden],
            WeaponType::Dagger => vec![
                Material::Bone,
                Material::Gold,
                Material::Iron,
                Material::Steel,
                Material::Stone,
            ],
            WeaponType::Dirk => vec![
                Material::Bone,
                Material::Iron,
                Material::Steel,
                Material::Stone,
            ],
            WeaponType::GreatSword => vec![
                Material::Bone,
                Material::Iron,
                Material::Steel,
                Material::Stone,
            ],
            WeaponType::Hammer => vec![Material::Iron, Material::Steel],
            WeaponType::LongSword => {
                vec![Material::Bone, Material::Iron, Material::Steel]
            }
            WeaponType::Mace => vec![Material::Iron, Material::Steel],
            WeaponType::Morningstar => vec![Material::Iron, Material::Steel],
            WeaponType::Shield => vec![
                Material::Hide,
                Material::Iron,
                Material::Leather,
                Material::Steel,
                Material::Wooden,
            ],
            WeaponType::ShortSword => vec![Material::Iron, Material::Steel],
            WeaponType::Whip => vec![Material::Leather],
        }
    }
}

impl TaggedObject for WeaponType {
    fn tags(&self) -> Vec<ObjectTag> {
        match *self {
            WeaponType::Buckler => vec![ObjectTag::Shield],
            WeaponType::Club => vec![ObjectTag::Blunt],
            WeaponType::Dagger => vec![ObjectTag::Blade],
            WeaponType::Dirk => vec![ObjectTag::Blade],
            WeaponType::GreatSword => vec![ObjectTag::Blade],
            WeaponType::Hammer => vec![ObjectTag::Blunt],
            WeaponType::LongSword => vec![ObjectTag::Blade],
            WeaponType::Mace => vec![ObjectTag::Blunt],
            WeaponType::Morningstar => vec![ObjectTag::Blunt],
            WeaponType::Shield => vec![ObjectTag::Shield],
            WeaponType::ShortSword => vec![ObjectTag::Blade],
            WeaponType::Whip => vec![ObjectTag::Rope, ObjectTag::Whip],
        }
    }
}

impl WeaponType {
    pub fn is_multiple(&self) -> bool {
        match *self {
            WeaponType::Club => false,
            WeaponType::Dagger => false,
            WeaponType::Hammer => false,
            WeaponType::LongSword => false,
            WeaponType::ShortSword => false,
            WeaponType::Buckler => false,
            WeaponType::Dirk => false,
            WeaponType::GreatSword => false,
            WeaponType::Mace => false,
            WeaponType::Morningstar => false,
            WeaponType::Shield => false,
            WeaponType::Whip => false,
        }
    }

    pub fn all() -> Vec<WeaponType> {
        WeaponType::into_enum_iter().collect()
    }
}

impl Equipment for WeaponType {
    fn possible_location_tags(&self) -> Vec<LocationTag> {
        match *self {
            WeaponType::Buckler => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Club => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Dagger => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Dirk => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::GreatSword => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Hammer => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::LongSword => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Mace => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Morningstar => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Shield => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::ShortSword => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WeaponType::Whip => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
        }
    }

    fn character_location_tags(&self) -> Vec<LocationTag> {
        match *self {
            WeaponType::Buckler => vec![LocationTag::Hand],
            WeaponType::Club => vec![LocationTag::Hand, LocationTag::Hip],
            WeaponType::Dagger => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
            WeaponType::Dirk => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
            WeaponType::GreatSword => vec![LocationTag::Hand, LocationTag::Back],
            WeaponType::Hammer => vec![LocationTag::Hand, LocationTag::Hip],
            WeaponType::LongSword => vec![
                LocationTag::Hand,
                LocationTag::Hip,
                LocationTag::HipSheath,
                LocationTag::Back,
            ],
            WeaponType::Mace => vec![LocationTag::Hand, LocationTag::Hip],
            WeaponType::Morningstar => vec![LocationTag::Hand, LocationTag::Hip],
            WeaponType::Shield => vec![LocationTag::Hand, LocationTag::Back],
            WeaponType::ShortSword => {
                vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath]
            }
            WeaponType::Whip => vec![LocationTag::Hand, LocationTag::Hip],
        }
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WeaponType::Buckler => write!(f, "buckler"),
            WeaponType::Club => write!(f, "club"),
            WeaponType::Dagger => write!(f, "dagger"),
            WeaponType::Dirk => write!(f, "dirk"),
            WeaponType::GreatSword => write!(f, "great sword"),
            WeaponType::Hammer => write!(f, "hammer"),
            WeaponType::LongSword => write!(f, "long sword"),
            WeaponType::Mace => write!(f, "mace"),
            WeaponType::Morningstar => write!(f, "morningstar"),
            WeaponType::Shield => write!(f, "shield"),
            WeaponType::ShortSword => write!(f, "short sword"),
            WeaponType::Whip => write!(f, "whip"),
        }
    }
}

#[cfg(test)]
mod weapon_type_tests {
    use crate::components::weapons::weapon_type::WeaponType;

    #[test]
    fn to_string() {
        assert_eq!("club", WeaponType::Club.to_string());
        assert_eq!("dagger", WeaponType::Dagger.to_string());
        assert_eq!("hammer", WeaponType::Hammer.to_string());
        assert_eq!("long sword", WeaponType::LongSword.to_string());
        assert_eq!("short sword", WeaponType::ShortSword.to_string());
    }
}
