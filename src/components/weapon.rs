#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use super::{
    attack::Attack,
    equipped_item::{Equippable, EquippedLocation},
    item_descriptor::ItemDescriptor,
    item_material::{BuiltWithMaterial, ItemMaterial}, descriptor_tags::{DescriptorTagged, DescriptorTag},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub struct Weapon {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub attack: Option<Attack>,
    pub weapon_type: WeaponType,
    pub descriptors: Vec<ItemDescriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<ItemMaterial>,
}

impl Equippable for Weapon {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation> {
        self.weapon_type.possible_equip_locations()
    }

    fn is_multiple(&self) -> bool {
        self.weapon_type.is_multiple()
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();
        for quality in self.descriptors.iter() {
            descriptions.push(quality.to_string());
        }

        if let Some(material) = &self.material {
            descriptions.push(material.to_string());
        }

        descriptions.push(self.weapon_type.to_string());

        write!(f, "{}", descriptions.join(" "))
    }
}

#[derive(Clone, Debug, PartialEq)]
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

impl DescriptorTagged for WeaponType {
    fn descriptor_tag(&self) -> DescriptorTag {
        match *self {
            WeaponType::Buckler => DescriptorTag::Shield,
            WeaponType::Club => DescriptorTag::Blunt,
            WeaponType::Dagger => DescriptorTag::Blade,
            WeaponType::Dirk => DescriptorTag::Blade,
            WeaponType::GreatSword => DescriptorTag::Blade,
            WeaponType::Hammer => DescriptorTag::Blunt,
            WeaponType::LongSword => DescriptorTag::Blade,
            WeaponType::Mace => DescriptorTag::Blunt,
            WeaponType::Morningstar => DescriptorTag::Blunt,
            WeaponType::Shield => DescriptorTag::Shield,
            WeaponType::ShortSword => DescriptorTag::Blade,
            WeaponType::Whip => DescriptorTag::Rope,
        }
    }
}

impl WeaponType {
    pub fn all() -> Vec<WeaponType> {
        vec![
            WeaponType::Buckler,
            WeaponType::Club,
            WeaponType::Dagger,
            WeaponType::Dirk,
            WeaponType::GreatSword,
            WeaponType::Hammer,
            WeaponType::LongSword,
            WeaponType::Mace,
            WeaponType::Morningstar,
            WeaponType::Shield,
            WeaponType::ShortSword,
            WeaponType::Whip,
        ]
    }

    pub fn possible_descriptors(&self) -> Vec<ItemDescriptor> {
        match *self {
            WeaponType::Club => vec![ItemDescriptor::Broken],
            WeaponType::Dagger => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Dull,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::Hammer => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Rusty,
            ],
            WeaponType::LongSword => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Dull,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::ShortSword => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Dull,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::Buckler => vec![
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
                ItemDescriptor::Broken,
            ],
            WeaponType::Dirk => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Dull,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::GreatSword => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Dull,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::Mace => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Rusty,
            ],
            WeaponType::Morningstar => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Chipped,
                ItemDescriptor::Rusty,
            ],
            WeaponType::Shield => vec![
                ItemDescriptor::Broken,
                ItemDescriptor::Rusty,
                ItemDescriptor::Shiny,
            ],
            WeaponType::Whip => Vec::new(),
        }
    }
}

impl Equippable for WeaponType {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation> {
        match *self {
            WeaponType::Club => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::Dagger => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::SheathedAtHip,
                EquippedLocation::HangingMoldySheath,
                EquippedLocation::StrappedToThigh,
            ],
            WeaponType::Hammer => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
            ],
            WeaponType::LongSword => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::SheathedAtHip,
                EquippedLocation::HangingMoldySheath,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::ShortSword => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::SheathedAtHip,
                EquippedLocation::HangingMoldySheath,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::Buckler => vec![EquippedLocation::AlmostFallingGrip],
            WeaponType::Dirk => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::SheathedAtHip,
                EquippedLocation::HangingMoldySheath,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::GreatSword => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::SheathedAtHip,
                EquippedLocation::HangingMoldySheath,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::Mace => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HeldLoosely,
            ],
            WeaponType::Morningstar => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HeldLoosely,
            ],
            WeaponType::Shield => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::HeldLoosely,
                EquippedLocation::StrappedToBack,
            ],
            WeaponType::Whip => vec![
                EquippedLocation::AlmostFallingGrip,
                EquippedLocation::ClenchedInFist,
                EquippedLocation::HangingHip,
                EquippedLocation::HeldLoosely,
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
    use crate::components::weapon::WeaponType;

    #[test]
    fn to_string() {
        assert_eq!("club", WeaponType::Club.to_string());
        assert_eq!("dagger", WeaponType::Dagger.to_string());
        assert_eq!("hammer", WeaponType::Hammer.to_string());
        assert_eq!("long sword", WeaponType::LongSword.to_string());
        assert_eq!("short sword", WeaponType::ShortSword.to_string());
    }
}

#[cfg(test)]
mod weapon_quality_tests {
    use crate::components::item_descriptor::ItemDescriptor;

    #[test]
    fn to_string() {
        assert_eq!("broken", ItemDescriptor::Broken.to_string());
        assert_eq!("chipped", ItemDescriptor::Chipped.to_string());
        assert_eq!("dull", ItemDescriptor::Dull.to_string());
        assert_eq!("rusty", ItemDescriptor::Rusty.to_string());
        assert_eq!("shiny", ItemDescriptor::Shiny.to_string());
    }
}

#[cfg(test)]
mod weapon_tests {
    use super::Weapon;

    #[test]
    fn to_string_without_qualities() {
        let weapon = Weapon {
            attack: None,
            weapon_type: super::WeaponType::LongSword,
            descriptors: Vec::new(),
            material: None,
        };

        assert_eq!("long sword", weapon.to_string());
    }

    #[test]
    fn to_string_with_qualities() {
        let weapon = Weapon {
            attack: None,
            weapon_type: super::WeaponType::LongSword,
            descriptors: vec![super::ItemDescriptor::Dull, super::ItemDescriptor::Chipped],
            material: None,
        };

        assert_eq!("dull chipped long sword", weapon.to_string());
    }
}
