use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum Size {
    Average,
    Huge,
    Large,
    Massive,
    Long,
    Medium,
    Narrow,
    Short,
    Small,
    Squat,
    Tall,
    Tiny,
    Wide,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Size::Average => "average",
            Size::Huge => "huge",
            Size::Long => "long",
            Size::Massive => "massive",
            Size::Medium => "medium",
            Size::Narrow => "narrow",
            Size::Short => "short",
            Size::Squat => "squat",
            Size::Tall => "tall",
            Size::Wide => "wide",
            Size::Large => "large",
            Size::Small => "small",
            Size::Tiny => "tiny",
        };

        write!(f, "{}", text)
    }
}

impl Size {
    pub fn is_average(&self) -> bool {
        matches!(*self, Size::Medium | Size::Average)
    }
}
