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
    Club,
    Dagger,
    Hammer,
    LongSword,
    ShortSword,
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
        }
    }

    fn is_multiple(&self) -> bool {
        match *self {
            WeaponType::Club => false,
            WeaponType::Dagger => false,
            WeaponType::Hammer => false,
            WeaponType::LongSword => false,
            WeaponType::ShortSword => false,
        }
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Club => write!(f, "club"),
            Self::Dagger => write!(f, "dagger"),
            Self::Hammer => write!(f, "hammer"),
            Self::LongSword => write!(f, "long sword"),
            Self::ShortSword => write!(f, "short sword"),
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
        };

        assert_eq!("dull chipped long sword", weapon.to_string());
    }
}
