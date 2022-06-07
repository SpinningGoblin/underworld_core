use crate::{
    actions::MovePlayerItem,
    components::player::PlayerCharacter,
    errors::Error,
    events::{Event, PlayerItemMoved},
    utils::ids::parse_id,
};

const MAX_WEAPONS_AT_READY: usize = 2;
const MAX_WEARABLES_AT_READY: usize = 8;

pub fn handle(
    move_player_item: &MovePlayerItem,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&move_player_item.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    if character_item.is_weapon()
        && player.character.count_weapons_at_ready() >= MAX_WEAPONS_AT_READY
        && move_player_item.put_at_the_ready
    {
        return Err(Error::TooManyWeaponsEquippedError);
    }

    if character_item.is_wearable()
        && player.character.count_wearables_at_ready() >= MAX_WEARABLES_AT_READY
        && move_player_item.put_at_the_ready
    {
        return Err(Error::TooManyWearablesEquippedError);
    }

    Ok(vec![Event::PlayerItemMoved(PlayerItemMoved {
        item_id,
        at_the_ready: move_player_item.put_at_the_ready,
        location: move_player_item.location_tag.clone(),
    })])
}
