use crate::{
    actions::OpenFixture,
    components::games::GameState,
    errors::Error,
    events::{Event, FixtureOpened},
    utils::ids::parse_id,
};

pub fn handle(open_fixture: &OpenFixture, state: &GameState) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&open_fixture.fixture_id)?;
    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    if !fixture_position.fixture.can_be_opened {
        return Err(Error::FixtureCannotBeOpened(fixture_id.to_string()));
    }

    Ok(vec![Event::FixtureOpened(FixtureOpened { fixture_id })])
}
