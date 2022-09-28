#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    LocationTag, {Item, ItemView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct CharacterItem {
    pub item: Item,
    pub equipped_location: LocationTag,
    pub at_the_ready: bool,
}

impl CharacterItem {
    pub fn is_at_the_ready(&self) -> bool {
        self.at_the_ready
    }

    pub fn is_packed(&self) -> bool {
        !self.at_the_ready
            || matches!(
                self.equipped_location,
                LocationTag::Packed
                    | LocationTag::Pockets
                    | LocationTag::Back
                    | LocationTag::Hip
                    | LocationTag::HipSheath
            )
    }

    pub fn is_in_hand(&self) -> bool {
        self.at_the_ready || self.equipped_location.eq(&LocationTag::Hand)
    }

    pub fn is_weapon(&self) -> bool {
        self.item.is_weapon()
    }

    pub fn is_wearable(&self) -> bool {
        self.item.is_wearable()
    }

    pub fn is_consumable(&self) -> bool {
        self.item.is_consumable()
    }

    pub fn decrease_uses(&mut self) {
        self.item.decrease_uses()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "CharacterItem"))]
pub struct CharacterItemView {
    pub item: ItemView,
    pub equipped_location: LocationTag,
    pub at_the_ready: bool,
}
