use crate::{
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::{Event, NpcDamagedByPoison, NpcPoisonEffectDurationChanged},
};

pub fn handle(state: &GameState, player: &PlayerCharacter) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    if let Some(poison_effect) = &player.character.current_effects.poison {
        events.push(Event::PlayerDamagedByPoison(poison_effect.damage));
        events.push(Event::PlayerPoisonDurationChanged(-1));
    }

    for npc in state
        .current_room()
        .npc_positions
        .iter()
        .map(|npc_position| &npc_position.npc)
    {
        if let Some(poison_effect) = &npc.character.current_effects.poison {
            events.push(Event::NpcDamagedByPoison(NpcDamagedByPoison {
                npc_id: npc.id,
                damage: poison_effect.damage,
            }));

            events.push(Event::NpcPoisonEffectDurationChanged(
                NpcPoisonEffectDurationChanged {
                    npc_id: npc.id,
                    duration: -1,
                },
            ));
        }
    }

    events
}
