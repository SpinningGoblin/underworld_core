use crate::{
    actions::SellPlayerItem,
    components::PlayerCharacter,
    errors::Error,
    events::{Event, PlayerItemRemoved},
    utils::ids::parse_id,
};

pub fn handle(
    sell_player_item: &SellPlayerItem,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&sell_player_item.item_id)?;
    let _ = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    Ok(vec![
        Event::PlayerItemRemoved(PlayerItemRemoved { item_id }),
        Event::PlayerGainedGold(1),
    ])
}
