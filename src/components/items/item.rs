use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{material::Material, tag::Tag, damage::{Attack, Defense}};

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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct ItemView {
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

impl Item {
    pub fn is_weapon(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_wearable())
    }

    pub fn look_at(&self, sees_full_item: bool, knows_all: bool) -> ItemView {
        let (descriptors, descriptors_known) = if sees_full_item || knows_all {
            (self.descriptors.clone(), true)
        } else {
            (Vec::new(), false)
        };

        let (material, material_known) = if sees_full_item || knows_all {
            (self.material.clone(), true)
        } else {
            (None, false)
        };

        let (attack, attack_known) = if knows_all {
            (self.attack.clone(), true)
        } else {
            (None, false)
        };

        let (defense, defense_known) = if knows_all {
            (self.defense.clone(), true)
        } else {
            (None, false)
        };

        ItemView {
            item_type: self.item_type.clone(),
            tags: self.tags.clone(),
            descriptors,
            descriptors_known,
            material,
            material_known,
            attack,
            attack_known,
            defense,
            defense_known,
        }
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
