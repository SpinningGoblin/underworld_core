#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::equipment::{
    location_descriptor::LocationDescriptor, location_tag::LocationTag,
};

use super::item::Item;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct CharacterItem {
    pub item: Item,
    pub is_hidden: bool,
    pub location_descriptor: LocationDescriptor,
    pub equipped_location_tags: Vec<LocationTag>,
    pub is_multiple: bool,
}

impl CharacterItem {
    pub fn is_equipped(&self) -> bool {
        self.equipped_location_tags.contains(&LocationTag::Equipped)
    }

    pub fn is_in_hand(&self) -> bool {
        self.equipped_location_tags.contains(&LocationTag::Equipped)
            && self.equipped_location_tags.contains(&LocationTag::Hand)
    }

    pub fn is_weapon(&self) -> bool {
        self.item.tags.iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.item.tags.iter().any(|tag| tag.is_wearable())
    }
}
