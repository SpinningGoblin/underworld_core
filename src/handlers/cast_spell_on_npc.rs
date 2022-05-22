use std::error::Error;

use crate::{
    actions::cast_spell_on_npc::CastSpellOnNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::spell_not_found_error::SpellNotFoundError,
    events::event::Event,
    utils::ids::parse_id,
};

pub fn handle_cast_spell_on_npc(
    cast_spell_on_npc: &CastSpellOnNpc,
    _state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let spell_id = parse_id(&cast_spell_on_npc.spell_id)?;
    let _ = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => return Err(Box::new(SpellNotFoundError(spell_id.to_string()))),
    };

    Ok(Vec::new())
}
