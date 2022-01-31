use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    equipped_item::{EquipLocationDescriptor, Equippable},
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
    fn possible_materials(&self) -> Vec<ItemMaterial> {
        match *self {
            WeaponType::Buckler => {
                vec![ItemMaterial::Hide, ItemMaterial::Iron, ItemMaterial::Steel]
            }
            WeaponType::Club => vec![
                ItemMaterial::Bone,
                ItemMaterial::Stone,
                ItemMaterial::Wooden,
            ],
            WeaponType::Dagger => vec![
                ItemMaterial::Bone,
                ItemMaterial::Gold,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
            ],
            WeaponType::Dirk => vec![
                ItemMaterial::Bone,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
            ],
            WeaponType::GreatSword => vec![
                ItemMaterial::Bone,
                ItemMaterial::Iron,
                ItemMaterial::Steel,
                ItemMaterial::Stone,
            ],
            WeaponType::Hammer => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WeaponType::LongSword => {
                vec![ItemMaterial::Bone, ItemMaterial::Iron, ItemMaterial::Steel]
            }
            WeaponType::Mace => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WeaponType::Morningstar => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WeaponType::Shield => vec![
                ItemMaterial::Hide,
                ItemMaterial::Iron,
                ItemMaterial::Leather,
                ItemMaterial::Steel,
                ItemMaterial::Wooden,
            ],
            WeaponType::ShortSword => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WeaponType::Whip => vec![ItemMaterial::Leather],
        }
    }
}

impl TaggedItem for WeaponType {
    fn tag(&self) -> ItemTag {
        match *self {
            WeaponType::Buckler => ItemTag::Shield,
            WeaponType::Club => ItemTag::Blunt,
            WeaponType::Dagger => ItemTag::Blade,
            WeaponType::Dirk => ItemTag::Blade,
            WeaponType::GreatSword => ItemTag::Blade,
            WeaponType::Hammer => ItemTag::Blunt,
            WeaponType::LongSword => ItemTag::Blade,
            WeaponType::Mace => ItemTag::Blunt,
            WeaponType::Morningstar => ItemTag::Blunt,
            WeaponType::Shield => ItemTag::Shield,
            WeaponType::ShortSword => ItemTag::Blade,
            WeaponType::Whip => ItemTag::Rope,
        }
    }
}

impl WeaponType {
    pub fn all() -> Vec<WeaponType> {
        WeaponType::into_enum_iter().collect()
    }
}

impl Equippable for WeaponType {
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
