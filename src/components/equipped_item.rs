#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct EquippedItem<T: Display + Clone + Debug + Equippable> {
    pub item: T,
    pub hidden: bool,
    pub equipped_location: EquippedLocation,
    pub multiple: bool,
}

pub trait Equippable {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation>;
    fn is_multiple(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum EquippedLocation {
    AlmostFallingGrip,
    ClenchedInFist,
    DanglingFromWrists,
    HangingHip,
    HangingLooselyShoulders,
    HeldLoosely,
    HangingMoldySheath,
    SheathedAtHip,
    StrappedToBack,
    StrappedToThigh,
    ClenchedInFists,
    None,
}

impl Default for EquippedLocation {
    fn default() -> Self {
        EquippedLocation::None
    }
}

impl EquippedLocation {
    pub fn unable_to_be_used_with(&self, other: &EquippedLocation) -> bool {
        match *self {
            EquippedLocation::AlmostFallingGrip => other.is_in_hand(),
            EquippedLocation::ClenchedInFist => other.is_in_hand(),
            EquippedLocation::DanglingFromWrists => false,
            EquippedLocation::HangingHip => other.is_at_hip(),
            EquippedLocation::HangingLooselyShoulders => false,
            EquippedLocation::HeldLoosely => other.is_in_hand(),
            EquippedLocation::HangingMoldySheath => other.is_at_hip(),
            EquippedLocation::SheathedAtHip => other.is_at_hip(),
            EquippedLocation::StrappedToBack => false,
            EquippedLocation::StrappedToThigh => false,
            EquippedLocation::ClenchedInFists => other.is_in_hand(),
            EquippedLocation::None => false,
        }
    }

    fn is_in_hand(&self) -> bool {
        matches!(
            *self,
            EquippedLocation::AlmostFallingGrip
                | EquippedLocation::ClenchedInFist
                | EquippedLocation::ClenchedInFists
                | EquippedLocation::HeldLoosely
        )
    }

    fn is_at_hip(&self) -> bool {
        matches!(
            *self,
            EquippedLocation::HangingHip
                | EquippedLocation::HangingMoldySheath
                | EquippedLocation::SheathedAtHip
        )
    }
}

impl Display for EquippedLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EquippedLocation::StrappedToThigh => write!(f, "strapped to its thigh"),
            EquippedLocation::ClenchedInFist => write!(f, "clenched in its fist"),
            EquippedLocation::HeldLoosely => write!(f, "held loosely"),
            EquippedLocation::StrappedToBack => write!(f, "strapped to its back"),
            EquippedLocation::SheathedAtHip => write!(f, "sheathed at its hip"),
            EquippedLocation::HangingMoldySheath => write!(f, "hanging in a moldy sheath"),
            EquippedLocation::HangingLooselyShoulders => {
                write!(f, "which hangs loosely around its shoulders")
            }
            EquippedLocation::DanglingFromWrists => write!(f, "dangling from its wrists"),
            EquippedLocation::HangingHip => write!(f, "hanging at its hip"),
            EquippedLocation::AlmostFallingGrip => write!(f, "almost falling from its grip"),
            EquippedLocation::ClenchedInFists => write!(f, "clenched in its fists"),
            EquippedLocation::None => write!(f, ""),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serialization")]
#[cfg(feature = "json")]
mod serialization_tests {
    use crate::components::{
        equipped_item::EquippedLocation,
        item_descriptor::ItemDescriptor,
        weapon::{Weapon, WeaponType},
    };

    use super::EquippedItem;

    #[test]
    fn serialize() {
        let weapon = Weapon {
            attack: None,
            weapon_type: crate::components::weapon::WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Dull],
            material: None,
        };
        let equipped_item = EquippedItem {
            item: weapon,
            hidden: false,
            equipped_location: EquippedLocation::StrappedToThigh,
            multiple: false,
        };
        let serialized = serde_json::to_string(&equipped_item).unwrap();
        let deserialized: EquippedItem<Weapon> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(WeaponType::LongSword, deserialized.item.weapon_type);
    }
}
