use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub enum NpcPositionDescriptor {
    GlaringAtYou,
    GlaringAtYouFromNearby,
    InCorner,
    InCornerStands,
    LoiteringAbout,
    StandingAround,
}

impl NpcPositionDescriptor {
    fn descriptor_position(&self) -> DescriptorPosition {
        match *self {
            NpcPositionDescriptor::GlaringAtYou => DescriptorPosition::Post,
            NpcPositionDescriptor::StandingAround => DescriptorPosition::Post,
            NpcPositionDescriptor::GlaringAtYouFromNearby => DescriptorPosition::Post,
            NpcPositionDescriptor::LoiteringAbout => DescriptorPosition::Post,
            NpcPositionDescriptor::InCorner => DescriptorPosition::Pre,
            NpcPositionDescriptor::InCornerStands => DescriptorPosition::Pre,
        }
    }

    pub fn unable_to_be_used_with(&self, other: NpcPositionDescriptor) -> bool {
        match *self {
            NpcPositionDescriptor::GlaringAtYou => other.is_post(),
            NpcPositionDescriptor::GlaringAtYouFromNearby => other.is_post(),
            NpcPositionDescriptor::InCorner => other.is_pre(),
            NpcPositionDescriptor::InCornerStands => other.is_pre(),
            NpcPositionDescriptor::LoiteringAbout => other.is_post(),
            NpcPositionDescriptor::StandingAround => other.is_post(),
        }
    }

    pub fn is_pre(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Pre
    }

    pub fn is_post(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Post
    }
}

impl Display for NpcPositionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NpcPositionDescriptor::GlaringAtYouFromNearby => {
                write!(f, "glaring at you from nearby")
            }
            NpcPositionDescriptor::LoiteringAbout => write!(f, "loitering about"),
            NpcPositionDescriptor::StandingAround => write!(f, "standing around"),
            NpcPositionDescriptor::InCorner => write!(f, "in the corner"),
            NpcPositionDescriptor::InCornerStands => write!(f, "in the corner stands"),
            NpcPositionDescriptor::GlaringAtYou => write!(f, "glaring at you"),
        }
    }
}
