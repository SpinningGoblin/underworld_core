use crate::{
    actions::CastSpellOnNpc,
    components::{
        games::game_state::GameState, player::PlayerCharacter, spells::spell_name::SpellName,
    },
    errors::Error,
    events::{event::Event, PlayerSpellForgotten, PlayerSpellUsed},
    utils::ids::parse_id,
};

use super::helpers::damage_npc;

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
    match learned_spell.spell.name {
        SpellName::ElectricBlast | SpellName::RagingFireball => {
            let damage = learned_spell.spell.damage();
            events.append(&mut damage_npc(player, npc, damage, true));
        }
        // TODO: There are non-damage spells that someone could cast on NPCs.
        _ => {}
    }

    events.push(Event::PlayerSpellUsed(PlayerSpellUsed { spell_id }));

    if learned_spell.spell.uses - 1 == 0 {
        events.push(Event::PlayerSpellForgotten(PlayerSpellForgotten {
            spell_id,
        }));
    }

    Ok(events)
}
