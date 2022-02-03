#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    attack::Attack,
    equipment::{location_tag::LocationTag, Equipment},
    material::Material,
    object::Object,
    object_descriptor::ObjectDescriptor,
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
    pub descriptors: Vec<ObjectDescriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
}

impl Equipment for Weapon {
    fn possible_location_tags(&self) -> Vec<LocationTag> {
        self.weapon_type.possible_location_tags()
    }

    fn character_location_tags(&self) -> Vec<LocationTag> {
        self.weapon_type.character_location_tags()
    }
}

impl Object for Weapon {
    fn look_at(&self, is_equipped: bool) -> String {
        self.describe(is_equipped)
    }

    fn material(&self) -> Option<Material> {
        self.material.clone()
    }

    fn is_multiple(&self) -> bool {
        self.weapon_type.is_multiple()
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
            descriptors: vec![
                super::ObjectDescriptor::Dull,
                super::ObjectDescriptor::Chipped,
            ],
            material: None,
        };

        assert_eq!("dull chipped long sword", weapon.to_string());
    }
}
