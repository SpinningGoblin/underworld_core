use std::error::Error;

use crate::{
    actions::move_player_item::MovePlayerItem,
    components::player::PlayerCharacter,
    errors::{
        item_not_found_error::ItemNotFoundError,
        too_many_weapons_equipped_error::TooManyWeaponsEquippedError,
    },
    events::event::{Event, PlayerItemMoved},
    utils::ids::parse_id,
};

pub fn handle(
    move_player_item: &MovePlayerItem,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let item_id = parse_id(&move_player_item.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Box::new(ItemNotFoundError(item_id.to_string()))),
    };

    if character_item.is_weapon() && player.character.count_weapons_at_ready() >= 2 {
        return Err(Box::new(TooManyWeaponsEquippedError));
    }

    Ok(vec![Event::PlayerItemMoved(PlayerItemMoved {
        item_id,
        at_the_ready: move_player_item.put_at_the_ready,
        location: move_player_item.location_tag.clone(),
    })])
}
