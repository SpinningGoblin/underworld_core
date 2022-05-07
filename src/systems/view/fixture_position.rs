use crate::components::rooms::fixture_position::{
    FixturePosition, FixturePositionView, FixturePositionViewArgs,
};

pub fn look_at(
    fixture_position: &FixturePosition,
    args: &FixturePositionViewArgs,
    knows_all: bool,
) -> FixturePositionView {
    FixturePositionView {
        group_descriptor: fixture_position.group_descriptor.clone(),
        fixtures: fixture_position
            .fixtures
            .iter()
            .map(|fixture| {
                super::fixture::look_at(
                    fixture,
                    args.knows_items,
                    args.knows_can_be_opened,
                    args.knows_has_hidden,
                    args.knows_hidden,
                    knows_all,
                )
            })
            .into_iter()
            .collect(),
        position_descriptors: fixture_position.position_descriptors.clone(),
    }
}
