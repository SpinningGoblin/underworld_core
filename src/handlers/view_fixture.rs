use crate::{
    actions::look_at_fixture::LookAtFixture,
    components::{fixtures::fixture::FixtureViewArgs, games::game_state::GameState},
    errors::Errors,
    events::{event::Event, fixture_viewed::FixtureViewed},
    systems::view::fixture,
    utils::ids::parse_id,
};

pub fn handle_view_fixture(
    action: &LookAtFixture,
    game_state: &GameState,
) -> Result<Vec<Event>, Errors> {
    let fixture_id = parse_id(&action.fixture_id)?;

    let fixture = match game_state.current_room().find_fixture(&fixture_id) {
        Some(it) => it,
        None => return Err(Errors::FixtureNotFound(fixture_id.to_string())),
    };

    let knowledge = game_state.fixture_knowledge(&fixture_id);

    let args = FixtureViewArgs {
        knows_items: knowledge.knows_items,
        knows_hidden: knowledge.knows_hidden_items,
        knows_has_hidden: knowledge.knows_has_hidden,
        knows_can_be_opened: knowledge.knows_can_be_opened,
    };

    let view = fixture::look_at(fixture, &args, game_state.player_knows_all);

    Ok(vec![Event::FixtureViewed(FixtureViewed {
        fixture_view: view,
    })])
}
