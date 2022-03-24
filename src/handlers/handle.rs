use crate::{
    actions::{action::Action, exit_room::ExitRoom},
    components::games::game::Game,
    events::{
        event::{apply_events, Event},
        room_exited::RoomExited,
        room_generated::RoomGenerated,
    },
    generators::{generator::Generator, rooms::random_room_generator},
    utils::ids::parse_id,
};

pub struct HandledAction {
    pub events: Vec<Event>,
    pub game: Game,
}

pub fn handle(action: &Action, game: &Game) -> HandledAction {
    let events = match action {
        Action::LookAtTarget(_) => Vec::new(),
        Action::LookAtRoom(_) => Vec::new(),
        Action::QuickLookRoom(_) => Vec::new(),
        Action::ExitRoom(exit_room) => handle_exit_room(exit_room, game),
    };

    let new_game = apply_events(&events, game);
    HandledAction {
        events,
        game: new_game,
    }
}

fn handle_exit_room(exit_room: &ExitRoom, game: &Game) -> Vec<Event> {
    // We need to check the exit maps for one with the room_id and exit.
    // If there's another exit id then find the room with that exit id and move
    // the player to that room.
    // If there is no room id as the other one, then we will need to generate
    // a room with this exit ID as its entrance.
    // Then we will need to move the player to that room and exit their
    // current room.

    let mut events: Vec<Event> = Vec::new();

    let exit_id = parse_id(&exit_room.exit_id).unwrap();
    let exit_map = game
        .world
        .exit_graph
        .iter()
        .find(|exit_map| exit_map.exit_id.eq(&exit_id))
        .unwrap();

    let other_room_id = if exit_map.left_room_id.eq(&Some(game.current_room_id)) {
        exit_map.right_room_id
    } else {
        exit_map.left_room_id
    };

    let room_id = match other_room_id {
        Some(id) => id,
        None => {
            let room_generator = random_room_generator(Some(exit_id));
            let room = room_generator.generate();
            let room_id = room.identifier.id;
            events.push(Event::RoomGenerated(RoomGenerated {
                room,
                entrance_id: exit_id,
            }));
            room_id
        }
    };

    events.push(Event::RoomExited(RoomExited {
        exit_id,
        old_room_id: game.current_room_id,
        new_room_id: room_id,
    }));

    events
}
