#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    item::{Item, ItemView},
    location_tag::LocationTag,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct CharacterItem {
    pub item: Item,
    pub is_hidden: bool,
    pub equipped_location: LocationTag,
    pub is_multiple: bool,
    pub at_the_ready: bool,
}

impl CharacterItem {
    pub fn is_equipped(&self) -> bool {
        self.at_the_ready
    }

    pub fn is_packed(&self) -> bool {
        self.equipped_location.eq(&LocationTag::Packed)
            || self.equipped_location.eq(&LocationTag::Pockets)
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
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct CharacterItemView {
    pub item: ItemView,
    pub is_hidden: Option<bool>,
    pub knows_equipped_location: bool,
    pub equipped_location: Option<LocationTag>,
    pub is_multiple: bool,
    pub at_the_ready: bool,
}

impl CharacterItemView {
    pub fn is_equipped(&self) -> bool {
        self.at_the_ready
    }

    pub fn is_weapon(&self) -> bool {
        self.item.is_weapon()
    }

    pub fn is_wearable(&self) -> bool {
        self.item.is_wearable()
    }
}
