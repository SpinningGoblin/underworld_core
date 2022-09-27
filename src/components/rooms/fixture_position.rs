#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::fixtures::{Fixture, FixtureView};

use super::fixture_position_descriptor::FixturePositionDescriptor;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixturePosition {
    pub fixture: Fixture,
    pub position_descriptor: Option<FixturePositionDescriptor>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "FixturePosition"))]
pub struct FixturePositionView {
    pub fixture: FixtureView,
    pub position_descriptor: Option<FixturePositionDescriptor>,
}
