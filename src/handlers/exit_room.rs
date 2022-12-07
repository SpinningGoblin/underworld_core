use crate::{
    actions::ExitRoom,
    components::games::GameState,
    errors::Error,
    events::{Event, RoomExited, RoomFirstSeen, RoomGenerated},
    generators::{generator::Generator, RoomGeneratorBuilder, RoomNpcGenerationArgs},
    utils::ids::parse_id,
};

pub fn handle(exit_room: &ExitRoom, state: &GameState) -> Result<Vec<Event>, Error> {
    // We need to check the exit maps for one with the room_id and exit.
    // If there's another exit id then find the room with that exit id and move
    // the player to that room.
    // If there is no room id as the other one, then we will need to generate
    // a room with this exit ID as its entrance.
    // Then we will need to move the player to that room and exit their
    // current room.

    let mut events: Vec<Event> = Vec::new();
    let exit_id = parse_id(&exit_room.exit_id)?;
    let exit_map = match state
        .world
        .exit_graph
        .iter()
        .find(|exit_map| exit_map.exit_id.eq(&exit_id))
    {
        Some(it) => it,
        None => return Err(Error::ExitNotFoundError(exit_id.to_string())),
    };

    let other_room_id = exit_map.other_room_id(state.current_room_id);
    let room_id = match other_room_id {
        Some(id) => id,
        None => {
            let room_generator = RoomGeneratorBuilder::new()
                .danger_level(state.danger_level)
                .entrance_id(exit_id)
                .room_npc_generation_args(RoomNpcGenerationArgs {
                    num_groups: None,
                    possible_species: None,
                    possible_life_modifiers: None,
                    allow_npcs_to_spawn_dead: None,
                    ghosts: Some(state.ghosts.to_vec()),
                })
                .build();
            let room = room_generator.generate();
            let room_id = room.id;
            events.push(Event::RoomGenerated(RoomGenerated {
                room,
                entrance_id: exit_id,
            }));
            room_id
        }
    };

    events.push(Event::RoomExited(RoomExited {
        exit_id,
        old_room_id: state.current_room_id,
        new_room_id: room_id,
    }));

    if !state.rooms_seen.contains(&room_id) {
        events.push(Event::RoomFirstSeen(RoomFirstSeen { room_id }));
        events.push(Event::GameDangerLevelIncreased(1));
        events.push(Event::PlayerMaxHealthChanged(1))
    }

    Ok(events)
}
