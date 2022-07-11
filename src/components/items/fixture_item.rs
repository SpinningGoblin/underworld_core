#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::items::{Item, ItemView};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixtureItem {
    pub item: Item,
    pub is_inside: bool,
    pub is_in_hidden_compartment: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "FixtureItem"))]
pub struct FixtureItemView {
    pub item: ItemView,
    pub is_inside: bool,
    pub is_in_hidden_compartment: Option<bool>,
    pub is_in_hidden_compartment_known: bool,
}
