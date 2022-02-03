#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

use super::{
    equipment::{location_descriptor::LocationDescriptor, location_tag::LocationTag, Equipment},
    item_tag::TaggedItem,
    object::Object,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct CharacterItem<T: Display + Clone + Debug + Object + Equipment> {
    pub item: T,
    pub is_hidden: bool,
    pub location_descriptor: LocationDescriptor,
    pub is_multiple: bool,
}

impl<T: Display + Clone + Debug + Object + Equipment + TaggedItem> CharacterItem<T> {
    pub fn is_equipped(&self) -> bool {
        self.location_descriptor
            .tags()
            .contains(&LocationTag::Equipped)
    }

    pub fn is_in_hand(&self) -> bool {
        self.location_descriptor
            .tags()
            .contains(&LocationTag::Equipped)
    }

    pub fn is_weapon(&self) -> bool {
        self.item.tags().iter().any(|tag| tag.is_weapon())
    }

    pub fn is_wearable(&self) -> bool {
        self.item.tags().iter().any(|tag| tag.is_wearable())
    }
}
