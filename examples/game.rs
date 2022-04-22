pub fn main() {
    #[cfg(feature = "serialization")]
    #[cfg(feature = "json")]
    {
        use underworld_core::{
            actions::{action::Action, attack_npc::AttackNpc, exit_room::ExitRoom},
            game::Game,
            generators::{game::game_generator, generator::Generator, players::player_generator},
        };

        let player = player_generator("Me", None, None, None).generate();
        let game_state = game_generator().generate();
        let mut game = Game {
            state: game_state,
            player,
        };
        let serialized_game = serde_json::to_string(&game.state).unwrap();
        println!("{}", &serialized_game);
        if let Some(first_exit) = game.state.current_room_exits().get(0) {
            let exit_room = ExitRoom {
                exit_id: first_exit.clone().to_string(),
            };

            let events = game.handle_action(&Action::ExitRoom(exit_room));
            let serialized_events = serde_json::to_string(&events).unwrap();
            println!("{}", &serialized_events);
            let serialized_new_game = serde_json::to_string(&game.state).unwrap();
            println!("{}", &serialized_new_game);

            if let Some(npc_positions) = &game.state.current_room().npc_positions.get(0) {
                if let Some(npc) = npc_positions.npcs.get(0) {
                    let attack = AttackNpc {
                        target_id: npc.identifier.id.to_string(),
                    };

                    let attack_events = game.handle_action(&Action::AttackNpc(attack));
                    let serialized_events = serde_json::to_string(&attack_events).unwrap();
                    println!("{}", &serialized_events);
                    let serialized_new_game = serde_json::to_string(&game.state).unwrap();
                    println!("{}", &serialized_new_game);

                    let serialized_player = serde_json::to_string(&game.player).unwrap();
                    println!("{}", &serialized_player);
                }
            }
        };
    }
}
