#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use super::{
    attack::Attack,
    equipped_item::{Equippable, EquippedLocation},
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
    pub descriptors: Vec<WeaponDescriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<WeaponMaterial>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WeaponMaterial {
    Bone,
    Gold,
    Iron,
    Leather,
    Steel,
    Stone,
    Wooden,
}

impl Display for WeaponMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WeaponMaterial::Bone => write!(f, "bone"),
            WeaponMaterial::Gold => write!(f, "gold"),
            WeaponMaterial::Iron => write!(f, "iron"),
            WeaponMaterial::Leather => write!(f, "leather"),
            WeaponMaterial::Steel => write!(f, "steel"),
            WeaponMaterial::Stone => write!(f, "stone"),
            WeaponMaterial::Wooden => write!(f, "wooden"),
        }
    }
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WeaponDescriptor {
    Broken,
    Chipped,
    Dull,
    Rusty,
    Shiny,
}

impl Display for WeaponDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Broken => write!(f, "broken"),
            Self::Chipped => write!(f, "chipped"),
            Self::Dull => write!(f, "dull"),
            Self::Rusty => write!(f, "rusty"),
            Self::Shiny => write!(f, "shiny"),
        }
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

    pub fn possible_descriptors(&self) -> Vec<WeaponDescriptor> {
        match *self {
            WeaponType::Club => vec![WeaponDescriptor::Broken],
            WeaponType::Dagger => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            WeaponType::Hammer => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Rusty,
            ],
            WeaponType::LongSword => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            WeaponType::ShortSword => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            WeaponType::Buckler => vec![
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
                WeaponDescriptor::Broken,
            ],
            WeaponType::Dirk => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            WeaponType::GreatSword => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            WeaponType::Mace => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Rusty,
            ],
            WeaponType::Morningstar => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Rusty,
            ],
            WeaponType::Shield => vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
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
    use crate::components::weapon::WeaponDescriptor;

    #[test]
    fn to_string() {
        assert_eq!("broken", WeaponDescriptor::Broken.to_string());
        assert_eq!("chipped", WeaponDescriptor::Chipped.to_string());
        assert_eq!("dull", WeaponDescriptor::Dull.to_string());
        assert_eq!("rusty", WeaponDescriptor::Rusty.to_string());
        assert_eq!("shiny", WeaponDescriptor::Shiny.to_string());
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
            descriptors: vec![
                super::WeaponDescriptor::Dull,
                super::WeaponDescriptor::Chipped,
            ],
            material: None,
        };

        assert_eq!("dull chipped long sword", weapon.to_string());
    }
}
