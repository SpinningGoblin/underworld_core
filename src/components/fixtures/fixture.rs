use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    identifier::{Identifier, IdentifierView},
    items::descriptor::Descriptor,
    material::Material,
    size::Size,
};

use super::fixture_type::FixtureType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Fixture {
    pub identifier: Identifier,
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub size: Size,
    pub descriptors: Vec<Descriptor>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct FixtureView {
    pub identifier: IdentifierView,
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub size: Size,
    pub descriptors: Vec<Descriptor>,
}

impl Display for FixtureView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}

impl FixtureView {
    pub fn describe(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if !self.size.is_average() {
            descriptions.push(format!("{}", &self.size));
        }

        match &self.material {
            Some(material) => descriptions.push(format!("{}", material)),
            None => {}
        }

        descriptions.push(format!("{}", &self.fixture_type));
        descriptions.join(" ")
    }
}

#[cfg(test)]
mod display_tests {
    use crate::{
        components::{
            fixtures::fixture_type::FixtureType, identifier::Identifier, material::Material,
        },
        systems::view::fixture::look_at,
    };

    use super::Fixture;

    #[test]
    fn display_with_material() {
        let fixture = Fixture {
            fixture_type: FixtureType::Chest,
            material: Some(Material::Steel),
            size: crate::components::size::Size::Average,
            descriptors: Vec::new(),
            identifier: Identifier::default(),
        };

        assert_eq!("steel chest", format!("{}", look_at(&fixture)));
    }
}
