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
    pub fixture: Fixture,
    pub position_descriptor: Option<FixturePositionDescriptor>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "FixturePosition"))]
pub struct FixturePositionView {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub fixture: FixtureView,
    pub position_descriptor: Option<FixturePositionDescriptor>,
}

impl Display for FixturePositionView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();

        match &self.position_descriptor {
            Some(descriptor) => {
                if descriptor.is_pre() {
                    parts.push(format!("{}", descriptor));
                }
            }
            None => {}
        };

        if let Some(group_descriptor) = &self.group_descriptor {
            parts.push(format!("{}", group_descriptor));
        }

        parts.push(self.fixtures_description());

        match &self.position_descriptor {
            Some(descriptor) => {
                if descriptor.is_post() {
                    parts.push(format!("{}", descriptor));
                }
            }
            None => {}
        };

        write!(f, "{}", parts.join(" "))
    }
}

impl FixturePositionView {
    pub fn display_as_sentence(&self) -> String {
        first_letter_to_upper_case(format!("{}.", self))
    }

    pub fn fixtures_description(&self) -> String {
        self.fixture.describe()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::{
        components::{
            fixtures::{fixture::Fixture, fixture_type::FixtureType},
            rooms::{
                fixture_position_descriptor::FixturePositionDescriptor,
                group_descriptor::GroupDescriptor,
            },
            size::Size,
        },
        systems::view::fixture_position::view,
    };

    use super::FixturePosition;

    #[test]
    fn display() {
        let table = Fixture {
            id: Uuid::new_v4(),
            name: None,
            fixture_type: FixtureType::Table,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
            items: Vec::new(),
            has_hidden_compartment: false,
            can_be_opened: false,
            open: false,
            hidden_compartment_open: false,
        };
        let _chair = Fixture {
            id: Uuid::new_v4(),
            name: None,
            fixture_type: FixtureType::Chair,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
            items: Vec::new(),
            has_hidden_compartment: false,
            can_be_opened: false,
            open: false,
            hidden_compartment_open: false,
        };
        let fixture_position = FixturePosition {
            group_descriptor: Some(GroupDescriptor::A),
            fixture: table,
            position_descriptor: Some(FixturePositionDescriptor::IsInTheCorner),
        };

        assert_eq!(
            "a table is in the corner",
            format!("{}", view(&fixture_position, &HashMap::new(), true))
        )
    }
}
