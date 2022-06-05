#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug, EnumIter, PartialEq)]
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
