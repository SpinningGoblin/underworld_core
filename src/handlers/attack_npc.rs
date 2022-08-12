use std::ops::RangeInclusive;

use rand::Rng;

use crate::{
    actions::AttackNpc,
    components::{damage::AttackEffect, games::GameState, PlayerCharacter, Species},
    errors::Error,
    events::{DeadNpcBeaten, Event, NpcItemDestroyed, NpcMissed, NpcPoisoned},
    utils::{ids::parse_id, rolls::roll_percent_succeeds},
};

use super::helpers::damage_npc;

const TOXIC_RANGE: RangeInclusive<i32> = 3..=6;
const TOXIC_DURATION_RANGE: RangeInclusive<i32> = 2..=4;

const ACID_DESTROYS_ITEM_CHANCE: i32 = 25;

pub fn handle(
    attack_npc: &AttackNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let mut events: Vec<Event> = Vec::new();
    let mut rng = rand::thread_rng();

    let room = state.current_room();
    let npc_id = parse_id(&attack_npc.npc_id)?;

    let npc = match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    if npc.character.is_dead() {
        events.push(Event::DeadNpcBeaten(DeadNpcBeaten {
            attacker_id: player.id,
            npc_id,
        }));
    } else if npc_will_dodge(&npc.character.species) {
        events.push(Event::NpcMissed(NpcMissed {
            attacker_id: player.id,
            npc_id,
        }));
    } else {
        let npc_defense = npc.character.full_defense();
        let player_attack = player.character.full_attack();
        let attack_damage = player_attack.attack_damage(&mut rng);
        let calculated_damage = npc_defense.calculate_damage_taken(&attack_damage);
        let damage = calculated_damage.min(npc.character.get_current_health());
        let (mut damage_events, npc_dead) = damage_npc(player, npc, damage);

        // If npc is alive, handle any attack effects on player weapons
        if !npc_dead {
            for effect in player_attack.effects.iter() {
                match effect {
                    AttackEffect::Toxic => {
                        events.push(Event::NpcPoisoned(NpcPoisoned {
                            npc_id: npc.id,
                            damage: rng.gen_range(TOXIC_RANGE),
                            duration: rng.gen_range(TOXIC_DURATION_RANGE),
                        }));
                    }
                    AttackEffect::Acidic => {
                        if roll_percent_succeeds(&mut rng, ACID_DESTROYS_ITEM_CHANCE) {
                            let equipped_items = npc.character.inventory.readied_weapons();
                            let index = rng.gen_range(0..equipped_items.len());
                            if let Some(character_item) = equipped_items.get(index) {
                                events.push(Event::NpcHitWithAcid(npc.id));
                                events.push(Event::NpcItemDestroyed(NpcItemDestroyed {
                                    npc_id: npc.id,
                                    item_id: character_item.item.id,
                                }))
                            }
                        }
                    }
                    AttackEffect::Sharp | AttackEffect::Crushing => {}
                }
            }
        }

        events.append(&mut damage_events);
    }

    Ok(events)
}

const PHANTOM_DODGE_CHANCE: i32 = 15;
const SHADOW_DODGE_CHANCE: i32 = 25;

fn npc_will_dodge(species: &Species) -> bool {
    let mut rng = rand::thread_rng();
    match *species {
        Species::Phantom => roll_percent_succeeds(&mut rng, PHANTOM_DODGE_CHANCE),
        Species::Shadow => roll_percent_succeeds(&mut rng, SHADOW_DODGE_CHANCE),
        _ => false,
    }
}
