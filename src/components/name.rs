#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use std::fmt::Display;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
