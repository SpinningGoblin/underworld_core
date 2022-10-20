use uuid::Uuid;

use crate::{
    actions::LootFixture,
    components::games::GameState,
    errors::Error,
    events::{Event, ItemTakenFromFixture},
    utils::ids::parse_id,
};

pub fn handle(loot_fixture: &LootFixture, state: &GameState) -> Result<Vec<Event>, Error> {
    let fixture_id = parse_id(&loot_fixture.fixture_id)?;
    let fixture_position = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Error::FixtureNotFoundError(fixture_id.to_string())),
    };

    let item_ids: Vec<Uuid> = loot_fixture
        .item_ids
        .iter()
        .flat_map(|i| match parse_id(i) {
            Ok(it) => Some(it),
            Err(_) => None,
        })
        .collect();
    let matching_items = fixture_position
        .fixture
        .items
        .iter()
        .filter(|fixture_item| item_ids.contains(&fixture_item.item.id));

    let mut events: Vec<Event> = Vec::new();
    for matching_item in matching_items {
        if (matching_item.is_in_hidden_compartment
            && !fixture_position.fixture.hidden_compartment_open)
            || (matching_item.is_inside && !fixture_position.fixture.open)
        {
            return Err(Error::ItemCannotBeTakenFromFixture(
                matching_item.item.id.to_string(),
            ));
        }

        events.push(Event::ItemTakenFromFixture(ItemTakenFromFixture {
            fixture_id,
            item_id: matching_item.item.id,
        }));
    }

    Ok(events)
}
