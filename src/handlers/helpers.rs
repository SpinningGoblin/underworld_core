use std::ops::RangeInclusive;

use rand::Rng;

use crate::{
    components::{damage::AttackEffect, NonPlayer, PlayerCharacter},
    events::{
        Event, NpcWeaponReadied, PlayerHit, PlayerHitNpc, PlayerKilled, PlayerKilledNpc,
        PlayerMissed, PlayerPoisoned,
    },
    utils::rolls::{roll_d6, roll_percent_succeeds},
};

const PLAYER_DODGE_CHANCE: i32 = 1;

const TOXIC_RANGE: RangeInclusive<i32> = 3..=6;
const TOXIC_DURATION_RANGE: RangeInclusive<i32> = 2..=4;

const ACID_DESTROYS_ITEM_CHANCE: i32 = 20;

pub fn npc_attack_player(
    player: &PlayerCharacter,
    npc: &NonPlayer,
    npc_can_ready: bool,
) -> Vec<Event> {
    let mut rng = rand::thread_rng();
    let dodge_roll = roll_d6(&mut rng, 1, 0);

    if dodge_roll <= PLAYER_DODGE_CHANCE {
        return vec![Event::PlayerMissed(PlayerMissed {
            attacker_id: npc.id,
        })];
    }

    let mut events: Vec<Event> = Vec::new();
    if npc.character.has_weapons_readied() {
        let npc_attack = npc.character.full_attack();
        let attack_damage = npc_attack.attack_damage(&mut rng);
        let player_defense = player.character.full_defense();
        let mut player_damage = player_defense.calculate_damage_taken(&attack_damage);
        if let Some(defense_aura) = &player.character.current_effects.shield_aura {
            let actual_damage = player_damage - defense_aura.damage_resistance;

            // Greater than 0, damage is higher than shield aura. Shield aura takes it and is dispelled
            // Otherwise shield takes all of the damage and is still there.
            if actual_damage >= 0 {
                events.push(Event::PlayerShieldAuraDamaged(
                    defense_aura.damage_resistance,
                ));
                events.push(Event::PlayerShieldAuraDissipated);
            } else {
                events.push(Event::PlayerShieldAuraDamaged(player_damage))
            }

            player_damage = actual_damage;
        }

        if player_damage > 0 {
            events.push(Event::PlayerHit(PlayerHit {
                attacker_id: npc.id,
                damage: player_damage,
            }));
        }

        if player_damage >= player.character.get_current_health() {
            events.push(Event::PlayerKilled(PlayerKilled {
                killer_id: Some(npc.id),
            }));

            if player.character.current_effects.resurrection_aura {
                events.push(Event::PlayerResurrected);
            }
        } else {
            // Handle any other attack effects that weren't previously handled.
            for effect in attack_damage.effects.iter() {
                match effect {
                    AttackEffect::Toxic => {
                        events.push(Event::PlayerPoisoned(PlayerPoisoned {
                            damage: rng.gen_range(TOXIC_RANGE),
                            duration: rng.gen_range(TOXIC_DURATION_RANGE),
                        }));
                    }
                    AttackEffect::Acidic => {
                        if roll_percent_succeeds(&mut rng, ACID_DESTROYS_ITEM_CHANCE) {
                            let equipped_items = player.character.inventory.readied_weapons();
                            let index = rng.gen_range(0..equipped_items.len());
                            if let Some(character_item) = equipped_items.get(index) {
                                events.push(Event::PlayerHitWithAcid);
                                events.push(Event::PlayerItemDestroyed(character_item.item.id));
                            }
                        }
                    }
                    AttackEffect::Sharp | AttackEffect::Crushing => {}
                }
            }
        }

        if let Some(retribution_aura) = &player.character.current_effects.retribution_aura {
            let mut rng = rand::thread_rng();
            let damage = retribution_aura.attack_roll(&mut rng);
            let (mut damage_events, _) = damage_npc(player, npc, damage);
            events.append(&mut damage_events);
            events.push(Event::PlayerRetributionAuraDissipated);
        }
    } else if npc_can_ready {
        // If there are no weapons readied, then all the NPC does is ready the weapon.
        let mut weapons = npc.character.inventory.non_readied_weapons();
        weapons.sort_by(|a, b| a.item.num_attack_rolls().cmp(&b.item.num_attack_rolls()));
        if let Some(weapon) = weapons.get(0) {
            events.push(Event::NpcWeaponReadied(NpcWeaponReadied {
                npc_id: npc.id,
                item_id: weapon.item.id,
            }));
        }
        if let Some(weapon) = weapons.get(1) {
            events.push(Event::NpcWeaponReadied(NpcWeaponReadied {
                npc_id: npc.id,
                item_id: weapon.item.id,
            }));
        }
    }
    events
}

pub fn damage_npc(player: &PlayerCharacter, npc: &NonPlayer, damage: i32) -> (Vec<Event>, bool) {
    let mut events: Vec<Event> = vec![Event::PlayerHitNpc(PlayerHitNpc {
        npc_id: npc.id,
        damage,
        attacker_id: player.id,
    })];

    let npc_dead = damage >= npc.character.get_current_health();

    if npc_dead {
        events.push(Event::PlayerKilledNpc(PlayerKilledNpc {
            npc_id: npc.id,
            killer_id: player.id,
        }));

        if !events
            .iter()
            .any(|event| matches!(event, Event::PlayerKilled(_)))
        {
            events.push(Event::GameDangerLevelIncreased(1));
            events.push(Event::PlayerMaxHealthChanged(1))
        }
    }

    (events, npc_dead)
}
