#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    item::{Item, ItemView},
    location_descriptor::LocationDescriptor,
    location_tag::LocationTag,
};

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
    pub fn look_at(&self, knows_hidden: bool, knows_all: bool) -> CharacterItemView {
        let full_item_hidden = self
            .equipped_location_tags
            .iter()
            .any(|tag| tag.hides_full_item());

        let is_hidden = if knows_hidden || knows_all {
            Some(self.is_hidden)
        } else {
            None
        };

        let (location_descriptor, knows_equipped_location, equipped_location_tags) =
            if self.is_hidden && (!knows_hidden || knows_all) {
                (None, false, Vec::new())
            } else {
                (
                    Some(self.location_descriptor.clone()),
                    true,
                    self.equipped_location_tags.clone(),
                )
            };

        CharacterItemView {
            item: self.item.look_at(!full_item_hidden, knows_all),
            is_hidden,
            location_descriptor,
            knows_equipped_location,
            equipped_location_tags,
            is_multiple: self.is_multiple,
        }
    }

    pub fn is_equipped(&self) -> bool {
        self.equipped_location_tags.contains(&LocationTag::Equipped)
    }

    pub fn is_packed(&self) -> bool {
        self.equipped_location_tags
            .iter()
            .any(|tag| tag == &LocationTag::Packed || tag == &LocationTag::Pockets)
    }

    pub fn is_in_hand(&self) -> bool {
        self.equipped_location_tags.contains(&LocationTag::Equipped)
            && self.equipped_location_tags.contains(&LocationTag::Hand)
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
    pub location_descriptor: Option<LocationDescriptor>,
    pub knows_equipped_location: bool,
    pub equipped_location_tags: Vec<LocationTag>,
    pub is_multiple: bool,
}

impl CharacterItemView {
    pub fn is_equipped(&self) -> bool {
        self.equipped_location_tags.contains(&LocationTag::Equipped)
    }

    pub fn is_weapon(&self) -> bool {
        self.item.is_weapon()
    }

    pub fn is_wearable(&self) -> bool {
        self.item.is_wearable()
    }
}
