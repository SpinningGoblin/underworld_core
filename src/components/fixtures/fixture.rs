use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::item_material::ItemMaterial;

use super::fixture_type::FixtureType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub struct Fixture {
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<ItemMaterial>,
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let material_description = match &self.material {
            Some(material) => format!("{}", material),
            None => "".to_string(),
        };

        write!(f, "{} {}", material_description, self.fixture_type)
    }
}

#[cfg(test)]
mod display_tests {
    use crate::components::{fixtures::fixture_type::FixtureType, item_material::ItemMaterial};

    use super::Fixture;

    #[test]
    fn display_with_material() {
        let fixture = Fixture {
            fixture_type: FixtureType::Chest,
            material: Some(ItemMaterial::Steel),
        };

        assert_eq!("steel chest", format!("{}", fixture));
    }
}
