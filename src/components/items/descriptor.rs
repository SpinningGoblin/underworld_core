use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
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
    IllFitting,
    LooseFitting,
    Ripped,
    Rotting,
    Rusty,
    Scuffed,
    SetOf,
    Shimmering,
    Shiny,
    Smoothed,
    Splintered,
    Stained,
    Tangled,
    Tarnished,
    Torn,
    WaterLogged,
    Weathered,
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Descriptor::Beaten => write!(f, "beaten"),
            Descriptor::Bleached => write!(f, "bleached"),
            Descriptor::Bloodstained => write!(f, "bloodstained"),
            Descriptor::Broken => write!(f, "broken"),
            Descriptor::Chipped => write!(f, "chipped"),
            Descriptor::Colourful => write!(f, "colourful"),
            Descriptor::Cracked => write!(f, "cracked"),
            Descriptor::Crumbling => write!(f, "crumbling"),
            Descriptor::Dingy => write!(f, "dingy"),
            Descriptor::Dirty => write!(f, "dirty"),
            Descriptor::Drab => write!(f, "drab"),
            Descriptor::Dull => write!(f, "dull"),
            Descriptor::IllFitting => write!(f, "ill fitting"),
            Descriptor::LooseFitting => write!(f, "loose fitting"),
            Descriptor::Ripped => write!(f, "ripped"),
            Descriptor::Rusty => write!(f, "rusty"),
            Descriptor::SetOf => write!(f, "set of"),
            Descriptor::Shimmering => write!(f, "shimmering"),
            Descriptor::Shiny => write!(f, "shiny"),
            Descriptor::Scuffed => write!(f, "scuffed"),
            Descriptor::Smoothed => write!(f, "smoothed"),
            Descriptor::Splintered => write!(f, "splintered"),
            Descriptor::Stained => write!(f, "stained"),
            Descriptor::Tangled => write!(f, "tangled"),
            Descriptor::Tarnished => write!(f, "tarnished"),
            Descriptor::Torn => write!(f, "torn"),
            Descriptor::WaterLogged => write!(f, "water logged"),
            Descriptor::Weathered => write!(f, "weathered"),
            Descriptor::Rotting => write!(f, "rotting"),
        }
    }
}
