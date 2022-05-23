use std::error::Error;

use crate::{
    actions::look_at_fixture::LookAtFixture,
    components::{fixtures::fixture::FixtureViewArgs, games::game_state::GameState},
    errors::FixtureNotFoundError,
    events::{Event, FixtureViewed},
    systems::view::fixture,
    utils::ids::parse_id,
};

pub fn handle(
    look_at_fixture: &LookAtFixture,
    state: &GameState,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let fixture_id = parse_id(&look_at_fixture.fixture_id)?;

    let fixture = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Box::new(FixtureNotFoundError(fixture_id.to_string()))),
    };

    let knowledge = state.fixture_knowledge(&fixture_id);

    let args = FixtureViewArgs {
        knows_items: knowledge.knows_items,
        knows_hidden: knowledge.knows_hidden_items,
        knows_has_hidden: knowledge.knows_has_hidden,
        knows_can_be_opened: knowledge.knows_can_be_opened,
    };

    let view = fixture::view(fixture, &args, state.player_knows_all);

    Ok(vec![Event::FixtureViewed(FixtureViewed {
        fixture_view: view,
    })])
}
