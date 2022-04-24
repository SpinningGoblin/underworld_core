use rand::Rng;

use crate::{
    actions::attack_npc::AttackNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::{
        event::Event, npc_hit::NpcHit, npc_killed::NpcKilled, player_hit::PlayerHit,
        player_killed::PlayerKilled, player_missed::PlayerMissed,
    },
    utils::ids::parse_id,
};

const PLAYER_DODGE_CHANCE: usize = 1;

pub fn handle_attack_npc(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    let room = state.current_room();
    if let Some(npc_id) = parse_id(&attack_npc.npc_id) {
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
