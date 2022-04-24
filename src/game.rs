use crate::{
    actions::action::Action,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Errors,
    events::event::Event,
    handlers::{handle, HandledAction},
};

pub struct Game {
    pub state: GameState,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn handle_action(&mut self, action: &Action) -> Result<Vec<Event>, Errors> {
        let HandledAction {
            events,
            new_state,
            new_player,
        } = handle(action, &self.state, &self.player)?;
        self.state = new_state;
        self.player = new_player;

        Ok(events)
    }
}
