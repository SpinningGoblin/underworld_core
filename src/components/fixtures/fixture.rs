use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{items::descriptor::Descriptor, material::Material, size::Size};

use super::fixture_type::FixtureType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Fixture {
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
pub struct FixtureView {
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub size: Size,
    pub descriptors: Vec<Descriptor>,
}

impl Fixture {
    pub fn look_at(&self) -> FixtureView {
        FixtureView {
            fixture_type: self.fixture_type.clone(),
            material: self.material.clone(),
            size: self.size.clone(),
            descriptors: self.descriptors.clone(),
        }
    }
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();

        if !self.size.is_average() {
            descriptions.push(format!("{}", &self.size));
        }

        match &self.material {
            Some(material) => descriptions.push(format!("{}", material)),
            None => {}
        }

        descriptions.push(format!("{}", &self.fixture_type));
        write!(f, "{}", descriptions.join(" "))
    }
}

#[cfg(test)]
mod display_tests {
    use crate::components::{fixtures::fixture_type::FixtureType, material::Material};

    use super::Fixture;

    #[test]
    fn display_with_material() {
        let fixture = Fixture {
            fixture_type: FixtureType::Chest,
            material: Some(Material::Steel),
            size: crate::components::size::Size::Average,
            descriptors: Vec::new(),
        };

        assert_eq!("steel chest", format!("{}", fixture));
    }
}
