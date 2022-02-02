#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    attack::Attack,
    equipped::equip_location_descriptor::EquipLocationDescriptor,
    item::{EquippableItem, Item},
    item_descriptor::ItemDescriptor,
    material::Material,
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
    pub material: Option<Material>,
}

impl EquippableItem for Weapon {
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor> {
        self.weapon_type.possible_equip_locations()
    }
}

impl Item for Weapon {
    fn is_multiple(&self) -> bool {
        self.weapon_type.is_multiple()
    }

    fn look_at(&self, is_equipped: bool) -> String {
        self.describe(is_equipped)
    }

    fn material(&self) -> Option<Material> {
        self.material.clone()
    }
}

impl Weapon {
    fn describe(&self, is_equipped: bool) -> String {
        let mut descriptions: Vec<String> = Vec::new();
        self.descriptors
            .iter()
            .filter(|descriptor| {
                if !is_equipped {
                    !descriptor.is_for_equipped()
                } else {
                    true
                }
            })
            .for_each(|descriptor| descriptions.push(descriptor.to_string()));

        if let Some(material) = &self.material {
            descriptions.push(material.to_string());
        }

        descriptions.push(self.weapon_type.to_string());

        descriptions.join(" ")
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe(false))
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
