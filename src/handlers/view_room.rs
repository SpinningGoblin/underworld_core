use std::{collections::HashMap, error::Error};

use uuid::Uuid;

use crate::{
    actions::look_at_current_room::LookAtCurrentRoom,
    components::{
        character::CharacterViewArgs, fixtures::fixture::FixtureViewArgs,
        games::game_state::GameState, non_player::NonPlayerViewArgs,
    },
    events::{event::Event, room_viewed::RoomViewed},
    systems::view::room::view,
};

pub fn handle(_: &LookAtCurrentRoom, state: &GameState) -> Result<Vec<Event>, Box<dyn Error>> {
    let room = state.current_room();

    let mut fixture_args: HashMap<Uuid, FixtureViewArgs> = HashMap::new();

    for fixture_id in room
        .fixture_positions
        .iter()
        .flat_map(|fixture_position| fixture_position.fixtures.iter())
        .map(|fixture| fixture.identifier.id)
    {
        let knowledge = state.fixture_knowledge(&fixture_id);

        fixture_args.insert(
            fixture_id,
            FixtureViewArgs {
                knows_items: knowledge.knows_items,
                knows_hidden: knowledge.knows_hidden_items,
                knows_has_hidden: knowledge.knows_has_hidden,
                knows_can_be_opened: knowledge.knows_can_be_opened,
            },
        );
    }

    let mut npc_args: HashMap<Uuid, NonPlayerViewArgs> = HashMap::new();

    for npc_id in room
        .npc_positions
        .iter()
        .map(|npc_position| &npc_position.npc)
        .map(|npc| npc.identifier.id)
    {
        let knowledge = state.npc_knowledge(&npc_id);
        npc_args.insert(
            npc_id,
            NonPlayerViewArgs {
                character_args: CharacterViewArgs {
                    knows_health: knowledge.knows_health,
                    knows_species: knowledge.knows_species,
                    knows_life_modifier: knowledge.knows_life_modifier,
                    knows_inventory: knowledge.knows_inventory,
                    knows_hidden_in_inventory: knowledge.knows_hidden_in_inventory,
                    knows_packed_in_inventory: knowledge.knows_packed_in_inventory,
                },
                knows_name: knowledge.knows_name,
            },
        );
    }

    let view = view(room, npc_args, fixture_args, state.player_knows_all);

    Ok(vec![Event::RoomViewed(RoomViewed { view })])
}
