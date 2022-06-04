use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum FixturePositionDescriptor {
    IsInTheCorner,
    SitsAlongOneSide,
    StandsInTheCorner,
    CrackedAndBrokenOnTheGround,
}

impl FixturePositionDescriptor {
    fn descriptor_position(&self) -> DescriptorPosition {
        match *self {
            FixturePositionDescriptor::IsInTheCorner => DescriptorPosition::Post,
            FixturePositionDescriptor::SitsAlongOneSide => DescriptorPosition::Post,
            FixturePositionDescriptor::StandsInTheCorner => DescriptorPosition::Post,
            FixturePositionDescriptor::CrackedAndBrokenOnTheGround => DescriptorPosition::Post,
        }
    }

    pub fn unable_to_be_used_with(&self, other: &FixturePositionDescriptor) -> bool {
        if self.is_post() {
            other.is_post()
        } else {
            other.is_pre()
        }
    }

    pub fn is_pre(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Pre
    }

    pub fn is_post(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Post
    }
}

impl Display for FixturePositionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            FixturePositionDescriptor::IsInTheCorner => "is in the corner",
            FixturePositionDescriptor::SitsAlongOneSide => "sits along one side",
            FixturePositionDescriptor::StandsInTheCorner => "stands in the corner",
            FixturePositionDescriptor::CrackedAndBrokenOnTheGround => {
                "cracked and broken on the ground"
            }
        };

        write!(f, "{}", text)
    }
}
