use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::fixture::{FixtureView, FixtureViewArgs},
    rooms::fixture_position::{FixturePosition, FixturePositionView},
};

pub fn look_at(
    fixture_position: &FixturePosition,
    args: &FixtureViewArgs,
    knows_all: bool,
) -> FixturePositionView {
    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixtures: fixture_position
            .fixtures
            .iter()
            .map(|fixture| super::fixture::look_at(fixture, args, knows_all))
            .into_iter()
            .collect(),
        position_descriptors: fixture_position.position_descriptors.clone(),
    }
}

pub fn view_v2(
    fixture_position: &FixturePosition,
    fixture_view_args: &HashMap<Uuid, FixtureViewArgs>,
    knows_all: bool,
) -> FixturePositionView {
    let fixtures: Vec<FixtureView> = fixture_position
        .fixtures
        .iter()
        .map(|fixture| {
            let args = fixture_view_args
                .get(&fixture.identifier.id)
                .cloned()
                .unwrap_or_default();
            super::fixture::look_at(fixture, &args, knows_all)
        })
        .into_iter()
        .collect();

    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixtures,
        position_descriptors: fixture_position.position_descriptors.clone(),
    }
}
