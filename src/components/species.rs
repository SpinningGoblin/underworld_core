#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
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
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum Species {
    Bugbear,
    Dragonkin,
    Frogkin,
    Goblin,
    Hobgoblin,
    Kobold,
    Lizardkin,
    Moblin,
    Ogre,
    Orc,
    Phantom,
    Rockoblin,
    Shadow,
    Turtlekin,
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Species::Bugbear => "bugbear",
            Species::Goblin => "goblin",
            Species::Kobold => "kobold",
            Species::Ogre => "ogre",
            Species::Orc => "orc",
            Species::Hobgoblin => "hobgoblin",
            Species::Shadow => "shadow",
            Species::Dragonkin => "dragonkin",
            Species::Frogkin => "frogkin",
            Species::Lizardkin => "lizardkin",
            Species::Phantom => "phantom",
            Species::Rockoblin => "rockoblin",
            Species::Moblin => "moblin",
            Species::Turtlekin => "turtlekin",
        };

        write!(f, "{}", text)
    }
}

impl Species {
    pub fn describe_count(&self, count: usize) -> String {
        if count > 1 {
            format!("{}s", self)
        } else {
            format!("{}", self)
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
        assert_eq!("shadow", Species::Shadow.to_string());
    }
}
