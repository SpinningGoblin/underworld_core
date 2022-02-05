use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub enum Size {
    Average,
    Huge,
    Massive,
    Long,
    Medium,
    Narrow,
    Short,
    Squat,
    Tall,
    Wide,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Size::Average => write!(f, "average"),
            Size::Huge => write!(f, "huge"),
            Size::Long => write!(f, "long"),
            Size::Massive => write!(f, "massive"),
            Size::Medium => write!(f, "medium"),
            Size::Narrow => write!(f, "narrow"),
            Size::Short => write!(f, "short"),
            Size::Squat => write!(f, "squat"),
            Size::Tall => write!(f, "tall"),
            Size::Wide => write!(f, "wide"),
        }
    }
}

impl Size {
    pub fn is_average(&self) -> bool {
        matches!(*self, Size::Medium | Size::Average)
    }
}
