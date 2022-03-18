use crate::components::rooms::fixture_position::{FixturePosition, FixturePositionView};

pub fn look_at(fixture_position: &FixturePosition) -> FixturePositionView {
    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixtures: fixture_position
            .fixtures
            .iter()
            .map(super::fixture::look_at)
            .into_iter()
            .collect(),
        position_descriptors: fixture_position.position_descriptors.clone(),
    }
}
