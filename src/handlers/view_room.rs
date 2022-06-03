use std::{collections::HashMap, error::Error};

use uuid::Uuid;

use crate::{
    actions::LookAtCurrentRoom,
    components::{
        character::CharacterViewArgs, fixtures::fixture::FixtureViewArgs,
        games::game_state::GameState, non_player::NonPlayerViewArgs,
    },
    events::{Event, RoomViewed},
    systems::view::room::view,
};

pub fn handle(_: &LookAtCurrentRoom, state: &GameState) -> Result<Vec<Event>, Box<dyn Error>> {
    let room = state.current_room();

    let mut fixture_args: HashMap<Uuid, FixtureViewArgs> = HashMap::new();

    for fixture_id in room
        .fixture_positions
        .iter()
        .flat_map(|fixture_position| fixture_position.fixtures.iter())
        .map(|fixture| fixture.id)
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
        .map(|npc| npc.id)
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

    let mut exit_visitations: HashMap<Uuid, bool> = HashMap::new();
    let room_id = room.id;
    for exit in room.exits.iter() {
        let exit_map = match state
            .world
            .exit_graph
            .iter()
            .find(|exit_map| exit_map.exit_id.eq(&exit.id))
        {
            Some(it) => it,
            None => continue,
        };

        let has_visited_other_room = exit_map.other_room_id(room_id).is_some();

        exit_visitations.insert(exit.id, has_visited_other_room);
    }

    let view = view(
        room,
        npc_args,
        fixture_args,
        exit_visitations,
        state.player_knows_all,
    );

    Ok(vec![Event::RoomViewed(RoomViewed { view })])
}
