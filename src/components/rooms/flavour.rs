use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::utils::sentences::first_letter_to_upper_case;

#[derive(Clone, Debug, IntoEnumIterator)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "openapi",
    derive(Enum),
    oai(rename_all = "snake_case", rename = "FlavourText")
)]
pub enum Flavour {
    AStrangeBreezeBlows,
    MoldMossCoversWalls,
    UnseenLightsFlickerWalls,
}

impl Display for Flavour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Flavour::AStrangeBreezeBlows => "a strange breeze blows",
            Flavour::MoldMossCoversWalls => "mold and moss cover the walls",
            Flavour::UnseenLightsFlickerWalls => "unseen lights flicker across the walls",
        };

        write!(f, "{}", text)
    }
}

impl Flavour {
    pub fn as_sentence(&self) -> String {
        first_letter_to_upper_case(format!("{}.", self))
    }
}
