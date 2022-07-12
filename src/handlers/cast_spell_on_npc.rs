use std::ops::RangeInclusive;

use rand::Rng;

use crate::{
    actions::CastSpellOnNpc,
    components::{games::GameState, player::PlayerCharacter, spells::SpellName},
    errors::Error,
    events::{
        Event, NpcPoisonEffectDurationChanged, NpcPoisonLevelChanged, NpcPoisoned,
        PlayerSpellForgotten, PlayerSpellUsed,
    },
    utils::ids::parse_id,
};

use super::helpers::damage_npc;

const POISON_DART_DAMAGE_RANGE: RangeInclusive<i32> = 2..=6;
const POISON_DART_DURATION_RANGE: RangeInclusive<i32> = 1..=4;

const POISON_CLOUD_DAMAGE_RANGE: RangeInclusive<i32> = 1..=8;
const POISON_CLOUD_DURATION_RANGE: RangeInclusive<i32> = 2..=5;

pub fn handle(
    cast_spell_on_npc: &CastSpellOnNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let spell_id = parse_id(&cast_spell_on_npc.spell_id)?;
    let learned_spell = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => return Err(Error::SpellNotFoundError(spell_id.to_string())),
    };

    let room = state.current_room();
    let npc_id = parse_id(&cast_spell_on_npc.npc_id)?;
    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    let mut events: Vec<Event> = Vec::new();

    events.push(Event::PlayerSpellUsed(PlayerSpellUsed { spell_id }));
    match learned_spell.spell.name {
        SpellName::ElectricBlast | SpellName::RagingFireball => {
            let spell_damage = learned_spell.spell.damage();
            let damage = spell_damage.min(npc.character.get_current_health());
            events.append(&mut damage_npc(player, npc, damage));
        }
        SpellName::PoisonDart => {
            if npc.character.current_effects.poison.is_none() {
                let mut rng = rand::thread_rng();
                let damage = rng.gen_range(POISON_CLOUD_DAMAGE_RANGE);
                let duration = rng.gen_range(POISON_CLOUD_DURATION_RANGE);

                events.push(Event::NpcPoisoned(NpcPoisoned {
                    npc_id,
                    damage,
                    duration,
                }));
            } else {
                let mut rng = rand::thread_rng();
                let damage = rng.gen_range(POISON_CLOUD_DAMAGE_RANGE);
                let duration = rng.gen_range(POISON_CLOUD_DURATION_RANGE);

                events.push(Event::NpcPoisonLevelChanged(NpcPoisonLevelChanged {
                    npc_id,
                    damage,
                }));
                events.push(Event::NpcPoisonDurationChanged(
                    NpcPoisonEffectDurationChanged { npc_id, duration },
                ));
            }
        }
        SpellName::PoisonCloud => {
            for npc_position in state.current_room().npc_positions.iter() {
                if npc_position.npc.character.current_effects.poison.is_none() {
                    let mut rng = rand::thread_rng();
                    let damage = rng.gen_range(POISON_DART_DAMAGE_RANGE);
                    let duration = rng.gen_range(POISON_DART_DURATION_RANGE);

                    events.push(Event::NpcPoisoned(NpcPoisoned {
                        npc_id: npc_position.npc.id,
                        damage,
                        duration,
                    }));
                } else {
                    let mut rng = rand::thread_rng();
                    let damage = rng.gen_range(POISON_DART_DAMAGE_RANGE);
                    let duration = rng.gen_range(POISON_DART_DURATION_RANGE);

                    events.push(Event::NpcPoisonLevelChanged(NpcPoisonLevelChanged {
                        npc_id: npc_position.npc.id,
                        damage,
                    }));
                    events.push(Event::NpcPoisonDurationChanged(
                        NpcPoisonEffectDurationChanged {
                            npc_id: npc_position.npc.id,
                            duration,
                        },
                    ));
                }
            }
        }
        // TODO: There are non-damage spells that someone could cast on NPCs.
        _ => {}
    }

    if learned_spell.spell.uses - 1 == 0 {
        events.push(Event::PlayerSpellForgotten(PlayerSpellForgotten {
            spell_id,
        }));
    }

    Ok(events)
}
