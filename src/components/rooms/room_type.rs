use std::{fmt::Display, ops::Range};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::dimension_descriptors::{HeightDescriptor, WidthDescriptor};

use super::room_descriptor::RoomDescriptor;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum RoomType {
    Cave,
    Cavern,
    PrisonCell,
    Room,
    EntryWay,
}

const SMALL: &str = "small";
const HUGE: &str = "huge";
const AVERAGE: &str = "";

const SMALL_ROOM_RANGE: Range<f32> = 1.5..1.8;
const NORMAL_ROOM_RANGE: Range<f32> = 2.5..3.5;
const LARGE_ROOM_RANGE: Range<f32> = 4.5..5.5;
const HUGE_ROOM_RANGE: Range<f32> = 9.0..12.0;

impl HeightDescriptor for RoomType {
    fn height_range(&self) -> std::ops::Range<f32> {
        match *self {
            Self::Cave => LARGE_ROOM_RANGE,
            Self::Cavern => HUGE_ROOM_RANGE,
            Self::PrisonCell => SMALL_ROOM_RANGE,
            Self::Room => NORMAL_ROOM_RANGE,
            Self::EntryWay => SMALL_ROOM_RANGE,
        }
    }

    fn bigger_text(&self) -> String {
        HUGE.to_string()
    }

    fn smaller_text(&self) -> String {
        SMALL.to_string()
    }

    fn average_text(&self) -> String {
        AVERAGE.to_string()
    }
}

const NARROW: &str = "narrow";
const WIDE: &str = "wide";

impl WidthDescriptor for RoomType {
    fn width_range(&self) -> Range<f32> {
        match *self {
            Self::Cave => LARGE_ROOM_RANGE,
            Self::Cavern => HUGE_ROOM_RANGE,
            Self::PrisonCell => SMALL_ROOM_RANGE,
            Self::Room => NORMAL_ROOM_RANGE,
            Self::EntryWay => SMALL_ROOM_RANGE,
        }
    }

    fn bigger_text(&self) -> String {
        WIDE.to_string()
    }

    fn smaller_text(&self) -> String {
        NARROW.to_string()
    }

    fn average_text(&self) -> String {
        AVERAGE.to_string()
    }
}

impl Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Cave => write!(f, "cave"),
            Self::Cavern => write!(f, "cavern"),
            Self::EntryWay => write!(f, "entryway"),
            Self::PrisonCell => write!(f, "prison cell"),
            Self::Room => write!(f, "room"),
        }
    }
}

impl RoomType {
    pub fn possible_descriptors(&self) -> Vec<RoomDescriptor> {
        match *self {
            RoomType::Cave => vec![
                RoomDescriptor::Chill,
                RoomDescriptor::Dark,
                RoomDescriptor::Dim,
                RoomDescriptor::Grimy,
                RoomDescriptor::Moist,
            ],
            RoomType::Cavern => vec![
                RoomDescriptor::Chill,
                RoomDescriptor::Dark,
                RoomDescriptor::Dim,
                RoomDescriptor::Grimy,
                RoomDescriptor::Moist,
            ],
            RoomType::PrisonCell => vec![
                RoomDescriptor::Chill,
                RoomDescriptor::Dark,
                RoomDescriptor::Dim,
                RoomDescriptor::Grimy,
                RoomDescriptor::Moist,
            ],
            RoomType::Room => vec![
                RoomDescriptor::Chill,
                RoomDescriptor::Dark,
                RoomDescriptor::Dim,
                RoomDescriptor::Grimy,
                RoomDescriptor::Moist,
            ],
            RoomType::EntryWay => vec![
                RoomDescriptor::Chill,
                RoomDescriptor::Dark,
                RoomDescriptor::Dim,
                RoomDescriptor::Grimy,
                RoomDescriptor::Moist,
            ],
        }
    }
}
