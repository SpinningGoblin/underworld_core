use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, IntoEnumIterator)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum RoomType {
    Cave,
    Cavern,
    PrisonCell,
    Room,
    EntryWay,
    TavernHall,
    Mausoleum,
    Cemetery,
    Crypt,
    TempleHall,
}

impl Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            RoomType::Cave => "cave",
            RoomType::Cavern => "cavern",
            RoomType::EntryWay => "entryway",
            RoomType::PrisonCell => "prison cell",
            RoomType::Room => "room",
            RoomType::TavernHall => "tavern hall",
            RoomType::Mausoleum => "mausoleum",
            RoomType::Cemetery => "cemetery",
            RoomType::Crypt => "crypt",
            RoomType::TempleHall => "temple hall",
        };

        write!(f, "{}", text)
    }
}
