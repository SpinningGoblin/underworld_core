use crate::{
    actions::OpenFixtureHiddenCompartment,
    components::games::GameState,
    errors::Error,
    events::{Event, FixtureHiddenCompartmentOpened},
    utils::ids::parse_id,
};

pub fn handle(
    open_fixture: &OpenFixtureHiddenCompartment,
    state: &GameState,
) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&open_fixture.fixture_id)?;
    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    let knowledge = state.fixture_knowledge(&fixture_id);

    if !knowledge.knows_has_hidden_compartment && !state.all_knowledge_unlocked {
        return Err(Error::FixtureHasHiddenCompartmentUnknown(
            fixture_id.to_string(),
        ));
    }

    if !fixture_position.fixture.has_hidden_compartment {
        return Err(Error::FixtureHasNoHiddenCompartment(fixture_id.to_string()));
    }

    Ok(vec![Event::FixtureHiddenCompartmentOpened(
        FixtureHiddenCompartmentOpened { fixture_id },
    )])
}
