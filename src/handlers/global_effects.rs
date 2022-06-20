use crate::{
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::Event,
};

pub fn handle(_: &GameState, _: &PlayerCharacter) -> Vec<Event> {
    Vec::new()
}
