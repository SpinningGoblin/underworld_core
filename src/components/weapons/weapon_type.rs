use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    equipped::equip_location_descriptor::EquipLocationDescriptor,
    item::{EquippableItem, Item},
    item_tag::{ItemTag, TaggedItem},
    material::{BuiltWithMaterial, Material},
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

impl TaggedItem for WeaponType {
    fn tags(&self) -> Vec<ItemTag> {
        match *self {
            WeaponType::Buckler => vec![ItemTag::Shield],
            WeaponType::Club => vec![ItemTag::Blunt],
            WeaponType::Dagger => vec![ItemTag::Blade],
            WeaponType::Dirk => vec![ItemTag::Blade],
            WeaponType::GreatSword => vec![ItemTag::Blade],
            WeaponType::Hammer => vec![ItemTag::Blunt],
            WeaponType::LongSword => vec![ItemTag::Blade],
            WeaponType::Mace => vec![ItemTag::Blunt],
            WeaponType::Morningstar => vec![ItemTag::Blunt],
            WeaponType::Shield => vec![ItemTag::Shield],
            WeaponType::ShortSword => vec![ItemTag::Blade],
            WeaponType::Whip => vec![ItemTag::Rope],
        }
    }
}

impl WeaponType {
    pub fn all() -> Vec<WeaponType> {
        WeaponType::into_enum_iter().collect()
    }
}

impl EquippableItem for WeaponType {
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor> {
        match *self {
            WeaponType::Club => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::Dagger => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::SheathedAtHip,
                EquipLocationDescriptor::HangingMoldySheath,
                EquipLocationDescriptor::StrappedToThigh,
            ],
            WeaponType::Hammer => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
            ],
            WeaponType::LongSword => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::SheathedAtHip,
                EquipLocationDescriptor::HangingMoldySheath,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::ShortSword => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::SheathedAtHip,
                EquipLocationDescriptor::HangingMoldySheath,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::Buckler => vec![EquipLocationDescriptor::AlmostFallingGrip],
            WeaponType::Dirk => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::SheathedAtHip,
                EquipLocationDescriptor::HangingMoldySheath,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::GreatSword => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::SheathedAtHip,
                EquipLocationDescriptor::HangingMoldySheath,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::Mace => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HeldLoosely,
            ],
            WeaponType::Morningstar => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HeldLoosely,
            ],
            WeaponType::Shield => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::HeldLoosely,
                EquipLocationDescriptor::StrappedToBack,
            ],
            WeaponType::Whip => vec![
                EquipLocationDescriptor::AlmostFallingGrip,
                EquipLocationDescriptor::ClenchedInFist,
                EquipLocationDescriptor::HangingHip,
                EquipLocationDescriptor::HeldLoosely,
            ],
        }
    }
}

impl Item for WeaponType {
    fn is_multiple(&self) -> bool {
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

    fn look_at(&self, _is_equipped: bool) -> String {
        format!("{}", self)
    }

    fn material(&self) -> Option<Material> {
        None
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
