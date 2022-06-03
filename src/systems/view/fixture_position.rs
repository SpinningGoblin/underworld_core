use std::collections::HashMap;

use uuid::Uuid;

use crate::components::{
    fixtures::fixture::{FixtureView, FixtureViewArgs},
    rooms::fixture_position::{FixturePosition, FixturePositionView},
};

pub fn view(
    fixture_position: &FixturePosition,
    fixture_view_args: &HashMap<Uuid, FixtureViewArgs>,
    knows_all: bool,
) -> FixturePositionView {
    let fixtures: Vec<FixtureView> = fixture_position
        .fixtures
        .iter()
        .map(|fixture| {
            let args = fixture_view_args
                .get(&fixture.id)
                .cloned()
                .unwrap_or_default();
            super::fixture::view(fixture, &args, knows_all)
        })
        .into_iter()
        .collect();

    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixtures,
        position_descriptors: fixture_position.position_descriptors.clone(),
    }
}
