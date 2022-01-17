use std::{fmt::Display, ops::Range};

use super::height_descriptor::HeightDescriptor;

#[derive(Clone, Debug)]
pub enum Species {
    Bugbear,
    Goblin,
    Kobold,
    Ogre,
    Orc,
    Unknown,
}

const SMALL_HEIGHT: Range<f32> = 0.6..1.2;
const MEDIUM_HEIGHT: Range<f32> = 1.2..2.0;
const LARGE_HEIGHT: Range<f32> = 2.05..4.4;
const UNKNOWN_HEIGHT: Range<f32> = 0.6..4.4;

const TALL: &str = "tall";
const SHORT: &str = "short";
const AVERAGE_HEIGHT: &str = "";

impl HeightDescriptor for Species {
    fn height_range(&self) -> Range<f32> {
        match *self {
            Self::Bugbear => MEDIUM_HEIGHT,
            Self::Goblin => SMALL_HEIGHT,
            Self::Kobold => SMALL_HEIGHT,
            Self::Ogre => LARGE_HEIGHT,
            Self::Orc => MEDIUM_HEIGHT,
            Self::Unknown => UNKNOWN_HEIGHT,
        }
    }

    fn bigger_text(&self) -> String {
        TALL.to_string()
    }

    fn smaller_text(&self) -> String {
        SHORT.to_string()
    }

    fn average_text(&self) -> String {
        AVERAGE_HEIGHT.to_string()
    }
}

impl Species {
    pub fn height_range(&self) -> Range<f32> {
        match *self {
            Self::Bugbear => MEDIUM_HEIGHT,
            Self::Goblin => SMALL_HEIGHT,
            Self::Kobold => SMALL_HEIGHT,
            Self::Ogre => LARGE_HEIGHT,
            Self::Orc => MEDIUM_HEIGHT,
            Self::Unknown => UNKNOWN_HEIGHT,
        }
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bugbear => write!(f, "Bugbear"),
            Self::Goblin => write!(f, "Goblin"),
            Self::Kobold => write!(f, "Kobold"),
            Self::Ogre => write!(f, "Ogre"),
            Self::Orc => write!(f, "Orc"),
            _ => write!(f, "Mysterious Entity"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::species::Species;

    #[test]
    fn to_string_when_bugbear() {
        assert_eq!("Bugbear", Species::Bugbear.to_string());
    }

    #[test]
    fn to_string_when_goblin() {
        assert_eq!("Goblin", Species::Goblin.to_string());
    }

    #[test]
    fn to_string_when_kobold() {
        assert_eq!("Kobold", Species::Kobold.to_string());
    }

    #[test]
    fn to_string_when_orc() {
        assert_eq!("Orc", Species::Orc.to_string());
    }

    #[test]
    fn to_string_when_unknown() {
        assert_eq!("Mysterious Entity", Species::Unknown.to_string());
    }
}
