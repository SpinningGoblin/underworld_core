use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::{
    components::fixtures::fixture::{Fixture, FixtureView},
    utils::sentences::first_letter_to_upper_case,
};

use super::{
    fixture_position_descriptor::FixturePositionDescriptor, group_descriptor::GroupDescriptor,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixturePosition {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub fixtures: Vec<Fixture>,
    pub position_descriptors: Vec<FixturePositionDescriptor>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct FixturePositionView {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub fixtures: Vec<FixtureView>,
    pub position_descriptors: Vec<FixturePositionDescriptor>,
}

impl Display for FixturePositionView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();

        for descriptor in self.position_descriptors.iter().filter(|d| d.is_pre()) {
            parts.push(format!("{}", descriptor));
        }

        if let Some(group_descriptor) = &self.group_descriptor {
            parts.push(format!("{}", group_descriptor));
        }

        parts.push(self.fixtures_description());

        for descriptor in self.position_descriptors.iter().filter(|d| d.is_post()) {
            parts.push(format!("{}", descriptor));
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl FixturePositionView {
    pub fn display_as_sentence(&self) -> String {
        first_letter_to_upper_case(format!("{}.", self))
    }

    pub fn fixtures_description(&self) -> String {
        let counts = crate::utils::frequencies::sorted_frequencies(
            self.fixtures.iter().map(|f| f.fixture_type.clone()),
        );

        let mut parts: Vec<String> = Vec::new();

        for (fixture, count) in counts {
            parts.push(fixture.describe_count(count));
        }

        parts.join(" and ")
    }
}

#[derive(Clone, Debug, Default)]
pub struct FixturePositionViewArgs {
    pub knows_items: bool,
    pub knows_hidden: bool,
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{
            fixtures::{fixture::Fixture, fixture_type::FixtureType},
            identifier::Identifier,
            rooms::{
                fixture_position_descriptor::FixturePositionDescriptor,
                group_descriptor::GroupDescriptor,
            },
            size::Size,
        },
        systems::view::fixture_position::look_at,
    };

    use super::{FixturePosition, FixturePositionViewArgs};

    #[test]
    fn display() {
        let table = Fixture {
            identifier: Identifier::default(),
            fixture_type: FixtureType::Table,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
            contained_items: Vec::new(),
            hidden_compartment_items: Vec::new(),
            has_hidden_compartment: false,
        };
        let chair = Fixture {
            identifier: Identifier::default(),
            fixture_type: FixtureType::Chair,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
            contained_items: Vec::new(),
            hidden_compartment_items: Vec::new(),
            has_hidden_compartment: false,
        };
        let fixture_position = FixturePosition {
            group_descriptor: Some(GroupDescriptor::A),
            fixtures: vec![table, chair.clone(), chair],
            position_descriptors: vec![FixturePositionDescriptor::IsInTheCorner],
        };

        assert_eq!(
            "a table and chairs is in the corner",
            format!(
                "{}",
                look_at(
                    &fixture_position,
                    &FixturePositionViewArgs {
                        knows_items: true,
                        knows_hidden: true,
                    },
                    false
                )
            )
        )
    }
}
