use crate::{
    components::{games::GameState, PlayerCharacter},
    events::{
        Event, NpcDamagedByPoison, NpcPoisonEffectDissipated, NpcPoisonEffectDurationChanged,
        PlayerKilledNpc,
    },
};

pub fn handle(state: &GameState, player: &PlayerCharacter) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    if let Some(poison_effect) = &player.character.current_effects.poison {
        events.push(Event::PlayerDamagedByPoison(poison_effect.damage));
        if poison_effect.duration - 1 <= 0 {
            events.push(Event::PlayerPoisonDissipated);
        } else {
            events.push(Event::PlayerPoisonDurationChanged(-1));
        }
    }

    for npc in state
        .current_room()
        .npc_positions
        .iter()
        .map(|npc_position| &npc_position.npc)
    {
        if let Some(poison_effect) = &npc.character.current_effects.poison {
            let damage = npc.character.get_current_health().min(poison_effect.damage);
            events.push(Event::NpcDamagedByPoison(NpcDamagedByPoison {
                npc_id: npc.id,
                damage,
            }));

            if damage >= npc.character.get_current_health() {
                events.push(Event::PlayerKilledNpc(PlayerKilledNpc {
                    killer_id: player.id,
                    npc_id: npc.id,
                }));

                events.push(Event::GameDangerLevelIncreased(1));
                events.push(Event::PlayerMaxHealthChanged(1))
            }

            if poison_effect.duration - 1 > 0 {
                events.push(Event::NpcPoisonDurationChanged(
                    NpcPoisonEffectDurationChanged {
                        npc_id: npc.id,
                        duration: -1,
                    },
                ));
            } else {
                events.push(Event::NpcPoisonEffectDissipated(
                    NpcPoisonEffectDissipated { npc_id: npc.id },
                ));
            }
        }
    }

    events
}
