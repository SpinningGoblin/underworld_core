#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "openapi",
    derive(Enum),
    oai(rename_all = "snake_case", rename = "ItemDescriptor")
)]
pub enum Descriptor {
    Beaten,
    Bleached,
    Bloodstained,
    Broken,
    Chipped,
    Colourful,
    Cracked,
    Crumbling,
    Dingy,
    Dirty,
    Drab,
    Dull,
    Heavy,
    Keen,
    Moldy,
    Quality,
    Ripped,
    Rotting,
    Rusty,
    Scuffed,
    Shimmering,
    Shiny,
    Smoothed,
    Soiled,
    Splintered,
    Stained,
    Tangled,
    Tarnished,
    Torn,
    WaterLogged,
    Weathered,
}
