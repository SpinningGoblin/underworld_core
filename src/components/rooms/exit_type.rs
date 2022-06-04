#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, EnumIter)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum ExitType {
    Door,
    HoleInTheWall,
    OpeningToTheVoid,
    HoleInTheFloor,
    StaircaseUp,
    StaircaseDown,
    Hallway,
    DugOutTunnelEntrance,
}

impl Display for ExitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            ExitType::Door => "door",
            ExitType::HoleInTheWall => "hole in the wall",
            ExitType::OpeningToTheVoid => "opening to the void",
            ExitType::HoleInTheFloor => "hole in the floor",
            ExitType::StaircaseUp => "staircase up",
            ExitType::StaircaseDown => "staircase down",
            ExitType::Hallway => "hallway",
            ExitType::DugOutTunnelEntrance => "dug out tunnel entrance",
        };

        write!(f, "{}", text)
    }
}
