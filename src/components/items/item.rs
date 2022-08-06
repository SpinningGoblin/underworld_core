#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{
    Material, Tag, {Attack, Defense},
};

use super::{
    Descriptor, ItemType, Throwable, ThrowableView, {Consumable, ConsumableView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Item {
    pub id: Uuid,
    pub name: Option<String>,
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
    pub consumable: Option<Consumable>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub throwable: Option<Throwable>,
}

impl Item {
    pub fn num_attack_rolls(&self) -> usize {
        self.attack
            .as_ref()
            .map(|attack| attack.num_rolls)
            .unwrap_or_default()
    }

    pub fn is_equippable(&self) -> bool {
        self.is_weapon() || self.is_wearable() || self.tags.iter().any(|tag| tag.is_equippable())
    }

    pub fn is_weapon(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_wearable())
    }

    pub fn is_consumable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_consumable())
    }

    pub fn decrease_uses(&mut self) {
        if let Some(mut consumable) = self.consumable.as_mut() {
            consumable.uses -= 1;
        }
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
    pub id: String,
    pub name: Option<String>,
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
    #[cfg_attr(feature = "serialization", serde(default))]
    pub consumable: Option<ConsumableView>,
    pub knows_consumable: bool,
    pub throwable: Option<ThrowableView>,
    pub is_equippable: bool,
}

impl ItemView {
    pub fn is_weapon(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.tags.iter().any(|tag| tag.is_wearable())
    }
}
