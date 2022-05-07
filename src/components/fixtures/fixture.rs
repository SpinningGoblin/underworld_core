use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    identifier::{Identifier, IdentifierView},
    items::{
        descriptor::Descriptor,
        item::{Item, ItemView},
    },
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
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<Descriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub contained_items: Vec<Item>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub hidden_compartment_items: Vec<Item>,
    pub has_hidden_compartment: bool,
    pub can_be_opened: bool,
    pub open: bool,
    pub hidden_compartment_open: bool,
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
    pub contained_items: Vec<ItemView>,
    pub knows_contained_items: bool,
    pub hidden_compartment_items: Vec<ItemView>,
    pub knows_hidden_compartment_items: bool,
    pub has_hidden_compartment: bool,
    pub knows_if_hidden_compartment: bool,
    pub open: bool,
    pub can_be_opened: bool,
    pub knows_if_can_be_opened: bool,
    pub hidden_compartment_open: bool,
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
            contained_items: Vec::new(),
            hidden_compartment_items: Vec::new(),
            has_hidden_compartment: false,
            can_be_opened: true,
            open: false,
            hidden_compartment_open: false,
        };

        assert_eq!(
            "steel chest",
            format!("{}", look_at(&fixture, true, true, true, true, true))
        );
    }
}
