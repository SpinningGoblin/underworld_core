use std::ops::RangeInclusive;

use rand::Rng;

use crate::{
    actions::CastSpellOnNpc,
    components::{games::GameState, spells::SpellName, PlayerCharacter},
    errors::Error,
    events::{
        Event, NpcItemDestroyed, NpcPoisonEffectDurationChanged, NpcPoisonLevelChanged,
        NpcPoisoned, PlayerSpellForgotten, PlayerSpellUsed,
    },
    utils::{ids::parse_id, rolls::roll_percent_succeeds},
};

use super::helpers::damage_npc;

const POISON_DART_DAMAGE_RANGE: RangeInclusive<i32> = 2..=6;
const POISON_DART_DURATION_RANGE: RangeInclusive<i32> = 1..=4;

const POISON_CLOUD_DAMAGE_RANGE: RangeInclusive<i32> = 1..=8;
const POISON_CLOUD_DURATION_RANGE: RangeInclusive<i32> = 2..=5;

const ACID_DESTROYS_ITEM_CHANCE: i32 = 75;

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
        SpellName::ElectricBlast => {
            let spell_damage = learned_spell.spell.damage();
            let damage = spell_damage.min(npc.character.get_current_health());
            let (mut damage_events, _) = damage_npc(player, npc, damage);
            events.append(&mut damage_events);
        }
        SpellName::RagingFireball => {
            let spell_damage = if npc.character.current_effects.covered_in_oil {
                learned_spell.spell.damage() * 2
            } else {
                learned_spell.spell.damage()
            };
            let damage = spell_damage.min(npc.character.get_current_health());
            let (mut damage_events, _) = damage_npc(player, npc, damage);
            events.append(&mut damage_events);
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
        SpellName::AcidSplash => {
            let mut rng = rand::thread_rng();
            if roll_percent_succeeds(&mut rng, ACID_DESTROYS_ITEM_CHANCE) {
                let equipped_items = npc.character.inventory.readied_weapons();
                let index = rng.gen_range(0..equipped_items.len());
                if let Some(character_item) = equipped_items.get(index) {
                    events.push(Event::NpcHitWithAcid(npc_id));
                    events.push(Event::NpcItemDestroyed(NpcItemDestroyed {
                        npc_id,
                        item_id: character_item.item.id,
                    }));
                }
            }
        }
        // TODO: There are non-damage spells that someone could cast on NPCs.
        // For now I'm going to have those be no-ops.
        SpellName::GreatHeal
        | SpellName::Heal
        | SpellName::Phoenix
        | SpellName::QuickHeal
        | SpellName::Retribution
        | SpellName::TinyShield => {}
    }

    if learned_spell.spell.uses - 1 == 0 {
        events.push(Event::PlayerSpellForgotten(PlayerSpellForgotten {
            spell_id,
        }));
    }

    Ok(events)
}
