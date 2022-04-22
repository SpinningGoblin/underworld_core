use rand::Rng;

use crate::{
    actions::{action::Action, attack_npc::AttackNpc, exit_room::ExitRoom},
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::{
        event::{apply_events, Event},
        npc_hit::NpcHit,
        npc_killed::NpcKilled,
        player_hit::PlayerHit,
        player_killed::PlayerKilled,
        player_missed::PlayerMissed,
        room_exited::RoomExited,
        room_generated::RoomGenerated,
    },
    generators::{generator::Generator, rooms::random_room_generator},
    utils::ids::parse_id,
};

pub struct HandledAction {
    pub events: Vec<Event>,
    pub new_player: PlayerCharacter,
    pub new_state: GameState,
}

pub fn handle(action: &Action, state: &GameState, player: &PlayerCharacter) -> HandledAction {
    let events = match action {
        Action::LookAtTarget(_) => Vec::new(),
        Action::LookAtRoom(_) => Vec::new(),
        Action::QuickLookRoom(_) => Vec::new(),
        Action::ExitRoom(exit_room) => handle_exit_room(exit_room, state),
        Action::AttackNpc(attack_npc) => handle_attack_npc(attack_npc, state, player),
    };

    let (new_state, new_player) = apply_events(&events, state, player);

    HandledAction {
        new_state,
        new_player,
        events,
    }
}

const PLAYER_DODGE_CHANCE: usize = 1;

fn handle_attack_npc(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    let room = state.current_room();
    if let Some(npc_id) = parse_id(&attack_npc.target_id) {
        if let Some(npc) = room.find_npc(&npc_id) {
            let defense = npc.character.defense();
            let attack = player.character.attack();
            let damage = (attack - defense).max(1);
            events.push(Event::NpcHit(NpcHit {
                npc_id,
                damage,
                attacker_id: player.identifier.id,
            }));

            if damage > npc.character.get_current_health().unwrap() {
                events.push(Event::NpcKilled(NpcKilled {
                    npc_id,
                    killer_id: player.identifier.id,
                }));
            } else {
                let mut rng = rand::thread_rng();
                let dodge_roll = rng.gen_range(1..=6);

                if dodge_roll <= PLAYER_DODGE_CHANCE {
                    events.push(Event::PlayerMissed(PlayerMissed {
                        attacker_id: npc.identifier.id,
                    }));
                } else {
                    let player_defense = player.character.defense();
                    let character_attack = npc.character.attack();
                    let player_damage = (character_attack - player_defense).max(1);
                    events.push(Event::PlayerHit(PlayerHit {
                        attacker_id: npc.identifier.id,
                        damage: player_damage,
                    }));
                    if player_damage > player.character.get_current_health().unwrap() {
                        events.push(Event::PlayerKilled(PlayerKilled {
                            killer_id: npc.identifier.id,
                        }));
                    }
                }
            }
        }
    }

    events
}

fn handle_exit_room(exit_room: &ExitRoom, state: &GameState) -> Vec<Event> {
    // We need to check the exit maps for one with the room_id and exit.
    // If there's another exit id then find the room with that exit id and move
    // the player to that room.
    // If there is no room id as the other one, then we will need to generate
    // a room with this exit ID as its entrance.
    // Then we will need to move the player to that room and exit their
    // current room.

    let mut events: Vec<Event> = Vec::new();

    let exit_id = parse_id(&exit_room.exit_id).unwrap();
    let exit_map = state
        .world
        .exit_graph
        .iter()
        .find(|exit_map| exit_map.exit_id.eq(&exit_id))
        .unwrap();

    let other_room_id = exit_map.other_room_id(state.current_room_id);

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
        old_room_id: state.current_room_id,
        new_room_id: room_id,
    }));

    events
}
