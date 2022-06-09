use crate::{
    components::{non_player::NonPlayer, player::PlayerCharacter},
    events::{
        Event, NpcWeaponReadied, PlayerHit, PlayerHitNpc, PlayerKilled, PlayerKilledNpc,
        PlayerMissed, PlayerResurrected, PlayerRetributionAuraDissipated,
    },
    utils::rolls::roll_d6,
};

pub fn npc_attack_player(
    player: &PlayerCharacter,
    npc: &NonPlayer,
    npc_can_ready: bool,
) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    if npc.character.has_weapons_readied() {
        let player_defense = player.character.defense();
        let character_attack = npc.character.attack();
        let player_damage = (character_attack - player_defense).max(1);

        // TODO Add handling for any defense aura player has.

        events.push(Event::PlayerHit(PlayerHit {
            attacker_id: npc.id,
            damage: player_damage,
            player_id: player.id,
        }));
        if player_damage >= player.character.get_current_health().unwrap() {
            events.push(Event::PlayerKilled(PlayerKilled { killer_id: npc.id }));

            if player.character.current_effects.resurrection_aura {
                events.push(Event::PlayerResurrected(PlayerResurrected));
            }
        }

        if let Some(retribution_aura) = &player.character.current_effects.retribution_aura {
            let mut rng = rand::thread_rng();
            let damage = retribution_aura.attack_roll(&mut rng);
            events.append(&mut damage_npc(player, npc, damage, false));
            events.push(Event::PlayerRetributionAuraDissipated(
                PlayerRetributionAuraDissipated,
            ))
        }
    } else if npc_can_ready {
        // If there are no weapons readied, then all the NPC does is ready the weapon.
        if let Some(character_item) = npc.character.strongest_non_readied_weapon() {
            events.push(Event::NpcWeaponReadied(NpcWeaponReadied {
                npc_id: npc.id,
                item_id: character_item.item.id,
            }));
        }
    }
    events
}

const PLAYER_DODGE_CHANCE: i32 = 1;

pub fn damage_npc(
    player: &PlayerCharacter,
    npc: &NonPlayer,
    damage: i32,
    can_counter: bool,
) -> Vec<Event> {
    let mut events: Vec<Event> = vec![Event::PlayerHitNpc(PlayerHitNpc {
        npc_id: npc.id,
        damage,
        attacker_id: player.id,
    })];

    let npc_dead = damage > npc.character.get_current_health().unwrap();

    // I always am going to have the NPC attack back if they can.
    // That way I can ensure the NPC will always get a shot.
    if can_counter {
        let mut rng = rand::thread_rng();
        let dodge_roll = roll_d6(&mut rng, 1, 0);

        if dodge_roll <= PLAYER_DODGE_CHANCE {
            events.push(Event::PlayerMissed(PlayerMissed {
                attacker_id: npc.id,
            }));
        } else {
            events.append(&mut npc_attack_player(player, npc, !npc_dead));
        }
    }

    if npc_dead {
        events.push(Event::PlayerKilledNpc(PlayerKilledNpc {
            npc_id: npc.id,
            killer_id: player.id,
        }));
    }

    events
}
