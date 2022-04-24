use crate::{
    components::{non_player::NonPlayer, player::PlayerCharacter},
    events::{event::Event, player_hit::PlayerHit, player_killed::PlayerKilled},
};

pub fn npc_attack_player(player: &PlayerCharacter, npc: &NonPlayer) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();
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

    events
}
