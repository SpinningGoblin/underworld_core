pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::{
            actions::{action::Action, exit_room::ExitRoom},
            generators::{game::game_generator, generator::Generator, players::player_generator},
            handlers::handle::handle,
        };

        let player = player_generator("Me", None, None, None).generate();
        let game = game_generator(player).generate();
        if let Some(first_exit) = game.current_room_exits().get(0) {
            let exit_room = ExitRoom {
                exit_id: first_exit.clone().to_string(),
            };
            let handled_action = handle(&Action::ExitRoom(exit_room), &game);
            let serialized_events = serde_json::to_string(&handled_action.events).unwrap();
            println!("{}", &serialized_events);
            let serialized_game = serde_json::to_string(&game).unwrap();
            println!("{}", &serialized_game);
            let serialized_new_game = serde_json::to_string(&handled_action.game).unwrap();
            println!("{}", &serialized_new_game);
        };
    }
}
