use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::size::Size;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Dimensions {
    pub height: Size,
    pub width: Size,
    pub length: Size,
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();

        if !self.height.is_average() {
            parts.push(format!(" {}", &self.height));
        }

        if !self.width.is_average() {
            parts.push(format!(" {}", &self.width));
        }

        if !self.length.is_average() {
            parts.push(format!(" {}", &self.length));
        }

        write!(f, "{}", parts.join(" "))
    }
}
