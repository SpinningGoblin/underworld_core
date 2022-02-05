#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Species {
    Bugbear,
    Goblin,
    Hobgoblin,
    Kobold,
    Ogre,
    Orc,
    Unknown,
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Species::Bugbear => write!(f, "bugbear"),
            Species::Goblin => write!(f, "goblin"),
            Species::Kobold => write!(f, "kobold"),
            Species::Ogre => write!(f, "ogre"),
            Species::Orc => write!(f, "orc"),
            Species::Hobgoblin => write!(f, "hobgoblin"),
            Species::Unknown => write!(f, "unknown creature"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::species::Species;

    #[test]
    fn to_string_when_bugbear() {
        assert_eq!("bugbear", Species::Bugbear.to_string());
    }

    #[test]
    fn to_string_when_goblin() {
        assert_eq!("goblin", Species::Goblin.to_string());
    }

    #[test]
    fn to_string_when_kobold() {
        assert_eq!("kobold", Species::Kobold.to_string());
    }

    #[test]
    fn to_string_when_orc() {
        assert_eq!("orc", Species::Orc.to_string());
    }

    #[test]
    fn to_string_when_unknown() {
        assert_eq!("unknown creature", Species::Unknown.to_string());
    }
}
