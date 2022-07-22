use crate::{
    actions::ThrowItemAtNpc,
    components::{games::GameState, PlayerCharacter},
    errors::Error,
    events::{Event, PlayerItemRemoved},
    utils::ids::parse_id,
};

pub fn handle(
    throw_item_at_npc: &ThrowItemAtNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&throw_item_at_npc.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    let room = state.current_room();
    let npc_id = parse_id(&throw_item_at_npc.npc_id)?;
    match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    let mut events: Vec<Event> = Vec::new();
    if let Some(throwable) = &character_item.item.throwable {
        if let Some(oil_effect) = &throwable.effect.oil_splash_effect {
            if oil_effect.covers_all_enemies {
                for npc_position in room.npc_positions.iter() {
                    events.push(Event::NpcCoveredInOil(npc_position.npc.id));
                }
            } else {
                events.push(Event::NpcCoveredInOil(npc_id));
            }
        }
    } else {
        return Err(Error::ItemNotThrowableError(item_id.to_string()));
    }

    events.push(Event::PlayerItemRemoved(PlayerItemRemoved { item_id }));

    Ok(events)
}
