use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{attack::Attack, defense::Defense, material::Material, tag::Tag};

use super::{descriptor::Descriptor, item_type::ItemType};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Item {
    pub item_type: ItemType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub tags: Vec<Tag>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<Descriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub attack: Option<Attack>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub defense: Option<Defense>,
}

impl Item {
    pub fn look_at(&self, is_equipped: bool) -> String {
        self.describe(is_equipped)
    }

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

        descriptions.push(self.item_type.to_string());

        descriptions.join(" ")
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe(false))
    }
}
