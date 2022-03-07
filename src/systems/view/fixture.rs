use crate::components::fixtures::fixture::{Fixture, FixtureView};

pub fn look_at(fixture: &Fixture) -> FixtureView {
    FixtureView {
        identifier: super::identifier::to_view(&fixture.identifier, true),
        fixture_type: fixture.fixture_type.clone(),
        material: fixture.material.clone(),
        size: fixture.size.clone(),
        descriptors: fixture.descriptors.clone(),
    }
}
