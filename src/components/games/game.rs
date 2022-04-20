use crate::components::player::PlayerCharacter;

use super::game_state::GameState;

pub struct Game {
    pub state: GameState,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn update_state(&mut self, state: GameState) {
        self.state = state;
    }
}
