use std::error::Error;

use uuid::Uuid;

use crate::{
    actions::LootFixture,
    components::games::game_state::GameState,
    errors::FixtureNotFoundError,
    events::{Event, ItemTakenFromFixture},
    utils::ids::parse_id,
};

pub fn handle(loot_fixture: &LootFixture, state: &GameState) -> Result<Vec<Event>, Box<dyn Error>> {
    let fixture_id = parse_id(&loot_fixture.fixture_id)?;
    let fixture = match state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Box::new(FixtureNotFoundError(fixture_id.to_string()))),
    };

    let item_ids: Vec<Uuid> = loot_fixture
        .item_ids
        .iter()
        .flat_map(|i| match parse_id(i) {
            Ok(it) => Some(it),
            Err(_) => None,
        })
        .collect();
    let matching_items = fixture
        .items
        .iter()
        .filter(|fixture_item| item_ids.contains(&fixture_item.item.identifier.id));

    // TODO: I should probably check to make sure the fixture is actually open here.
    // But for now I'm just going to blindly move the items from the fixture to the player.

    let mut events: Vec<Event> = Vec::new();
    for matching_item in matching_items {
        events.push(Event::ItemTakenFromFixture(ItemTakenFromFixture {
            fixture_id,
            item_id: matching_item.item.identifier.id,
        }));
    }

    Ok(events)
}
