use crate::{
    actions::LookAtFixture,
    components::{fixtures::FixtureViewArgs, games::GameState},
    errors::Error,
    events::{Event, FixtureViewed},
    systems::view::fixture,
    utils::ids::parse_id,
};

pub fn handle(look_at_fixture: &LookAtFixture, state: &GameState) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&look_at_fixture.fixture_id)?;

    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    let knowledge = state.fixture_knowledge(&fixture_id);

    let args = FixtureViewArgs {
        knows_has_hidden_compartment: knowledge.knows_has_hidden_compartment,
    };

    let view = fixture::view(
        &fixture_position.fixture,
        &args,
        state.all_knowledge_unlocked,
    );

    Ok(vec![Event::FixtureViewed(FixtureViewed {
        fixture_view: view,
    })])
}
