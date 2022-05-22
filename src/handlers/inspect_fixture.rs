use std::error::Error;

use crate::{
    actions::inspect_fixture::InspectFixture,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::fixture_not_found_error::FixtureNotFoundError,
    events::{
        event::Event, fixture_can_be_opened_discovered::FixtureCanBeOpenedDiscovered,
        fixture_contained_discovered::FixtureContainedDiscovered,
        fixture_has_hidden_discovered::FixtureHasHiddenDiscovered,
        fixture_hidden_items_discovered::FixtureHiddenItemsDiscovered,
    },
    utils::{ids::parse_id, rolls::roll_d6},
};

use super::helpers::npc_attack_player;

const DISCOVER_HIDDEN_CHANCE: i32 = 3;
const DISCOVER_CONTAINED_CHANCE: i32 = 5;
const DISCOVER_CAN_BE_OPENED_CHANCE: i32 = 1;
const DISCOVER_HIDDEN_ITEMS_CHANCE: i32 = 5;
const NPC_ATTACKS_CHANCE: i32 = 5;

pub fn handle_inspect_fixture(
    inspect_fixture: &InspectFixture,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();
    let fixture_id = parse_id(&inspect_fixture.fixture_id)?;

    if state.current_room().find_fixture(&fixture_id).is_none() {
        return Err(Box::new(FixtureNotFoundError(fixture_id.to_string())));
    }

    let mut rng = rand::thread_rng();

    if inspect_fixture.discover_can_be_opened
        && roll_d6(&mut rng, 1, 0) >= DISCOVER_CAN_BE_OPENED_CHANCE
    {
        events.push(Event::FixtureCanBeOpenedDiscovered(
            FixtureCanBeOpenedDiscovered { fixture_id },
        ));
    }

    if inspect_fixture.discover_contained && roll_d6(&mut rng, 1, 0) >= DISCOVER_CONTAINED_CHANCE {
        events.push(Event::FixtureContainedDiscovered(
            FixtureContainedDiscovered { fixture_id },
        ));
    }

    if inspect_fixture.discover_hidden && roll_d6(&mut rng, 1, 0) >= DISCOVER_HIDDEN_CHANCE {
        events.push(Event::FixtureHasHiddenDiscovered(
            FixtureHasHiddenDiscovered { fixture_id },
        ));

        if inspect_fixture.discover_hidden_items
            && roll_d6(&mut rng, 1, 0) >= DISCOVER_HIDDEN_ITEMS_CHANCE
        {
            events.push(Event::FixtureHiddenItemsDiscovered(
                FixtureHiddenItemsDiscovered { fixture_id },
            ));
        }
    }

    if roll_d6(&mut rng, 1, 0) >= NPC_ATTACKS_CHANCE {
        match state.current_room().first_alive_npc() {
            Some(it) => events.append(&mut npc_attack_player(player, it)),
            None => {}
        };
    }

    Ok(events)
}
