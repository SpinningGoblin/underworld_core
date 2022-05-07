use crate::components::{
    fixtures::fixture::FixtureViewArgs,
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
