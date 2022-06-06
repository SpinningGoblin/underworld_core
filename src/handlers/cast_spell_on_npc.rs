use crate::{
    actions::CastSpellOnNpc,
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::{Error, SpellNotFoundError},
    events::event::Event,
    utils::ids::parse_id,
};

pub fn handle(
    cast_spell_on_npc: &CastSpellOnNpc,
    _state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let spell_id = parse_id(&cast_spell_on_npc.spell_id)?;
    let _ = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => {
            return Err(Error::SpellNotFoundError(SpellNotFoundError(
                spell_id.to_string(),
            )))
        }
    };

    Ok(Vec::new())
}
