#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    attack::Attack,
    equipped_item::{Equippable, EquippedLocation},
    item_descriptor::ItemDescriptor,
    item_material::ItemMaterial,
};

use super::weapon_type::WeaponType;

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
