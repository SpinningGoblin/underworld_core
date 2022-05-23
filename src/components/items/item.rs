use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    damage::{Attack, Defense},
    identifier::{Identifier, IdentifierView},
    material::Material,
    tag::Tag,
};

use super::{consumable_effect::ConsumableEffect, descriptor::Descriptor, item_type::ItemType};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Item {
    pub identifier: Identifier,
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
    #[cfg_attr(feature = "serialization", serde(default))]
    pub consumable_effect: Option<ConsumableEffect>,
}

impl Item {
    pub fn num_attack_rolls(&self) -> usize {
        self.attack
            .as_ref()
            .map(|attack| attack.num_rolls)
            .unwrap_or_default()
    }

    pub fn is_weapon(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_wearable())
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Item"))]
pub struct ItemView {
    pub identifier: IdentifierView,
    pub item_type: ItemType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub tags: Vec<Tag>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<Descriptor>,
    pub descriptors_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub material_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub attack: Option<Attack>,
    pub attack_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub defense: Option<Defense>,
    pub defense_known: bool,
}

impl ItemView {
    pub fn is_weapon(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_wearable())
    }

    pub fn describe(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();
        self.descriptors
            .iter()
            .for_each(|descriptor| descriptions.push(descriptor.to_string()));

        if let Some(material) = &self.material {
            descriptions.push(material.to_string());
        }

        descriptions.push(self.item_type.to_string());
        descriptions.join(" ")
    }
}

impl Display for ItemView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
