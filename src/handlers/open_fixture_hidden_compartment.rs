use crate::{
    actions::OpenFixtureHiddenCompartment,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Error,
    events::{Event, FixtureHiddenCompartmentOpened},
    utils::ids::parse_id,
};

use super::helpers::first_npc_possibly_attacks;

pub fn handle(
    open_fixture: &OpenFixtureHiddenCompartment,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&open_fixture.fixture_id)?;
    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    let knowledge = state.fixture_knowledge(&fixture_id);

    if !knowledge.knows_has_hidden_compartment {
        return Err(Error::FixtureHasHiddenCompartmentUnknown(
            fixture_id.to_string(),
        ));
    }

    if !fixture_position.fixture.has_hidden_compartment {
        return Err(Error::FixtureHasNoHiddenCompartment(fixture_id.to_string()));
    }

    let mut events: Vec<Event> = vec![Event::FixtureHiddenCompartmentOpened(
        FixtureHiddenCompartmentOpened { fixture_id },
    )];

    events.append(&mut first_npc_possibly_attacks(player, state));

    Ok(events)
}
