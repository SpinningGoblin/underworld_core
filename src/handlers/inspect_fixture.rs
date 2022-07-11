use crate::{
    actions::InspectFixture,
    components::games::GameState,
    errors::Error,
    events::{Event, FixtureHasHiddenCompartmentDiscovered},
    utils::{ids::parse_id, rolls::roll_d6},
};

const DISCOVER_HIDDEN_COMPARTMENT_CHANCE: i32 = 2;

pub fn handle(inspect_fixture: &InspectFixture, state: &GameState) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = Vec::new();
    let fixture_id = parse_id(&inspect_fixture.fixture_id)?;

    if state.current_room().find_fixture(&fixture_id).is_none() {
        return Err(Error::FixtureNotFoundError(fixture_id.to_string()));
    }

    let mut rng = rand::thread_rng();

    if inspect_fixture.discover_hidden_compartment
        && roll_d6(&mut rng, 1, 0) >= DISCOVER_HIDDEN_COMPARTMENT_CHANCE
    {
        events.push(Event::FixtureHasHiddenCompartmentDiscovered(
            FixtureHasHiddenCompartmentDiscovered { fixture_id },
        ));
    }

    Ok(events)
}
