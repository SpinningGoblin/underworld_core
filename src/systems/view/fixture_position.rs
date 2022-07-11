use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::FixtureViewArgs,
    rooms::{FixturePosition, FixturePositionView},
};

pub fn view(
    fixture_position: &FixturePosition,
    fixture_view_args: &HashMap<Uuid, FixtureViewArgs>,
    knows_all: bool,
) -> FixturePositionView {
    let args = fixture_view_args
        .get(&fixture_position.fixture.id)
        .cloned()
        .unwrap_or_default();
    let fixture = super::fixture::view(&fixture_position.fixture, &args, knows_all);

    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixture,
        position_descriptor: fixture_position.position_descriptor.clone(),
    }
}
