use crate::{
    actions::OpenFixture,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Error,
    events::{Event, FixtureOpened},
    utils::ids::parse_id,
};

use super::helpers::first_npc_possibly_attacks;

pub fn handle(
    open_fixture: &OpenFixture,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&open_fixture.fixture_id)?;
    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    if !fixture_position.fixture.can_be_opened {
        return Err(Error::FixtureCannotBeOpened(fixture_id.to_string()));
    }

    let mut events: Vec<Event> = vec![Event::FixtureOpened(FixtureOpened { fixture_id })];

    events.append(&mut first_npc_possibly_attacks(player, state));

    Ok(events)
}
