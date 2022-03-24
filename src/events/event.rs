#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::games::game::Game;

use super::{room_exited::RoomExited, room_generated::RoomGenerated};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Event {
    RoomExited(RoomExited),
    RoomGenerated(RoomGenerated),
}

pub fn apply_events(events: &[Event], game: &Game) -> Game {
    let mut new_game = game.clone();

    for event in events.iter() {
        match event {
            Event::RoomExited(room_exited) => {
                new_game.current_room_id = room_exited.new_room_id;
                new_game.rooms_seen.push(room_exited.new_room_id);
            }
            Event::RoomGenerated(room_generated) => new_game
                .world
                .add_room(room_generated.entrance_id, room_generated.room.clone()),
        }
    }

    new_game
}
