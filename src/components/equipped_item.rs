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
    pub equipped_location: EquipLocationDescriptor,
    pub multiple: bool,
}

pub trait Equippable {
    fn look_at(&self, is_equipped: bool) -> String;
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor>;
    fn is_multiple(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum EquipLocationDescriptor {
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

impl Default for EquipLocationDescriptor {
    fn default() -> Self {
        EquipLocationDescriptor::None
    }
}

impl EquipLocationDescriptor {
    pub fn unable_to_be_used_with(&self, other: &EquipLocationDescriptor) -> bool {
        match *self {
            EquipLocationDescriptor::AlmostFallingGrip => other.is_in_hand(),
            EquipLocationDescriptor::ClenchedInFist => other.is_in_hand(),
            EquipLocationDescriptor::DanglingFromWrists => false,
            EquipLocationDescriptor::HangingHip => other.is_at_hip(),
            EquipLocationDescriptor::HangingLooselyShoulders => false,
            EquipLocationDescriptor::HeldLoosely => other.is_in_hand(),
            EquipLocationDescriptor::HangingMoldySheath => other.is_at_hip(),
            EquipLocationDescriptor::SheathedAtHip => other.is_at_hip(),
            EquipLocationDescriptor::StrappedToBack => false,
            EquipLocationDescriptor::StrappedToThigh => false,
            EquipLocationDescriptor::ClenchedInFists => other.is_in_hand(),
            EquipLocationDescriptor::None => false,
        }
    }

    fn is_in_hand(&self) -> bool {
        matches!(
            *self,
            EquipLocationDescriptor::AlmostFallingGrip
                | EquipLocationDescriptor::ClenchedInFist
                | EquipLocationDescriptor::ClenchedInFists
                | EquipLocationDescriptor::HeldLoosely
        )
    }

    fn is_at_hip(&self) -> bool {
        matches!(
            *self,
            EquipLocationDescriptor::HangingHip
                | EquipLocationDescriptor::HangingMoldySheath
                | EquipLocationDescriptor::SheathedAtHip
        )
    }
}

impl Display for EquipLocationDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EquipLocationDescriptor::StrappedToThigh => write!(f, "strapped to its thigh"),
            EquipLocationDescriptor::ClenchedInFist => write!(f, "clenched in its fist"),
            EquipLocationDescriptor::HeldLoosely => write!(f, "held loosely"),
            EquipLocationDescriptor::StrappedToBack => write!(f, "strapped to its back"),
            EquipLocationDescriptor::SheathedAtHip => write!(f, "sheathed at its hip"),
            EquipLocationDescriptor::HangingMoldySheath => write!(f, "hanging in a moldy sheath"),
            EquipLocationDescriptor::HangingLooselyShoulders => {
                write!(f, "which hangs loosely around its shoulders")
            }
            EquipLocationDescriptor::DanglingFromWrists => write!(f, "dangling from its wrists"),
            EquipLocationDescriptor::HangingHip => write!(f, "hanging at its hip"),
            EquipLocationDescriptor::AlmostFallingGrip => write!(f, "almost falling from its grip"),
            EquipLocationDescriptor::ClenchedInFists => write!(f, "clenched in its fists"),
            EquipLocationDescriptor::None => write!(f, ""),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serialization")]
#[cfg(feature = "json")]
mod serialization_tests {
    use crate::components::{
        equipped_item::EquipLocationDescriptor,
        item_descriptor::ItemDescriptor,
        weapons::{weapon::Weapon, weapon_type::WeaponType},
    };

    use super::EquippedItem;

    #[test]
    fn serialize() {
        let weapon = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Dull],
            material: None,
        };
        let equipped_item = EquippedItem {
            item: weapon,
            hidden: false,
            equipped_location: EquipLocationDescriptor::StrappedToThigh,
            multiple: false,
        };
        let serialized = serde_json::to_string(&equipped_item).unwrap();
        let deserialized: EquippedItem<Weapon> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(WeaponType::LongSword, deserialized.item.weapon_type);
    }
}
