use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug, IntoEnumIterator)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Descriptor {
    Chill,
    Dark,
    Dim,
    Grimy,
    Moist,
    AStrangeBreezeBlows,
    MoldMossCoversWalls,
    UnseenLightsFlickerWalls,
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Descriptor::Chill => write!(f, "chill"),
            Descriptor::Dark => write!(f, "dark"),
            Descriptor::Dim => write!(f, "dim"),
            Descriptor::Grimy => write!(f, "grimy"),
            Descriptor::Moist => write!(f, "moist"),
            Descriptor::AStrangeBreezeBlows => write!(f, "a strange breeze blows"),
            Descriptor::MoldMossCoversWalls => write!(f, "mold and moss cover the walls"),
            Descriptor::UnseenLightsFlickerWalls => {
                write!(f, "unseen lights flicker across the walls")
            }
        }
    }
}

impl Descriptor {
    pub fn get_position(&self) -> DescriptorPosition {
        match *self {
            Descriptor::Chill => DescriptorPosition::Pre,
            Descriptor::Dark => DescriptorPosition::Pre,
            Descriptor::Dim => DescriptorPosition::Pre,
            Descriptor::Grimy => DescriptorPosition::Pre,
            Descriptor::Moist => DescriptorPosition::Pre,
            Descriptor::AStrangeBreezeBlows => DescriptorPosition::Post,
            Descriptor::MoldMossCoversWalls => DescriptorPosition::Post,
            Descriptor::UnseenLightsFlickerWalls => DescriptorPosition::Post,
        }
    }

    pub fn is_pre(&self) -> bool {
        self.get_position() == DescriptorPosition::Pre
    }

    pub fn as_sentence(&self) -> String {
        first_letter_to_upper_case(format!("{}.", self))
    }
}

fn first_letter_to_upper_case(s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
