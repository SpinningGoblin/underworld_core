use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum GroupDescriptor {
    A,
    AFew,
    AGangOf,
    AGroupOf,
    ALone,
    ASingle,
    Some,
}

impl Display for GroupDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GroupDescriptor::AFew => write!(f, "a few"),
            GroupDescriptor::AGangOf => write!(f, "a gang of"),
            GroupDescriptor::AGroupOf => write!(f, "a group of"),
            GroupDescriptor::ALone => write!(f, "a lone"),
            GroupDescriptor::ASingle => write!(f, "a single"),
            GroupDescriptor::Some => write!(f, "some"),
            GroupDescriptor::A => write!(f, "a"),
        }
    }
}
