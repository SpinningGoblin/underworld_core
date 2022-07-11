#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::items::{CharacterItem, CharacterItemView};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub equipment: Vec<CharacterItem>,
}

impl Inventory {
    pub fn count_weapons_at_ready(&self) -> usize {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.at_the_ready)
            .count()
    }

    pub fn count_wearables_at_ready(&self) -> usize {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.at_the_ready)
            .count()
    }

    pub fn find_item(&self, item_id: &Uuid) -> Option<CharacterItem> {
        self.equipment
            .iter()
            .find(|character_item| character_item.item.id.eq(item_id))
            .cloned()
    }

    pub fn add_item(&mut self, character_item: CharacterItem) {
        self.equipment.push(character_item)
    }

    pub fn remove_item(&mut self, item_id: &Uuid) -> Option<CharacterItem> {
        let index = self
            .equipment
            .iter()
            .enumerate()
            .find(|(_, character_item)| character_item.item.id.eq(item_id))
            .map(|(index, _)| index);

        match index {
            Some(it) => Some(self.equipment.remove(it)),
            None => None,
        }
    }

    pub fn equipped_wearables(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.is_at_the_ready())
            .cloned()
            .collect()
    }

    pub fn readied_weapons(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.is_at_the_ready())
            .cloned()
            .collect()
    }

    pub fn non_readied_weapons(&self) -> Vec<&CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && !item.is_at_the_ready())
            .collect()
    }

    pub fn strongest_non_readied_weapon(&self) -> Option<&CharacterItem> {
        self.non_readied_weapons()
            .into_iter()
            .max_by(|a, b| a.item.num_attack_rolls().cmp(&b.item.num_attack_rolls()))
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Inventory"))]
pub struct InventoryView {
    pub equipment: Vec<CharacterItemView>,
}

impl InventoryView {
    pub fn equipped_wearables(&self) -> Vec<CharacterItemView> {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.is_equipped())
            .cloned()
            .collect()
    }

    pub fn equipped_weapons(&self) -> Vec<CharacterItemView> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.is_equipped())
            .cloned()
            .collect()
    }
}
