use crate::{
    actions::MovePlayerItem,
    components::player::PlayerCharacter,
    errors::{Error, ItemNotFoundError, TooManyWeaponsEquippedError},
    events::{Event, PlayerItemMoved},
    utils::ids::parse_id,
};

pub fn handle(
    move_player_item: &MovePlayerItem,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&move_player_item.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => {
            return Err(Error::ItemNotFoundError(ItemNotFoundError(
                item_id.to_string(),
            )))
        }
    };

    if character_item.is_weapon()
        && player.character.count_weapons_at_ready() >= 2
        && move_player_item.put_at_the_ready
    {
        return Err(Error::TooManyWeaponsEquippedError(
            TooManyWeaponsEquippedError,
        ));
    }

    Ok(vec![Event::PlayerItemMoved(PlayerItemMoved {
        item_id,
        at_the_ready: move_player_item.put_at_the_ready,
        location: move_player_item.location_tag.clone(),
    })])
}
