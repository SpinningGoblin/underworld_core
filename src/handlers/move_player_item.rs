use crate::{
    actions::move_player_item::MovePlayerItem,
    components::player::PlayerCharacter,
    errors::Errors,
    events::{event::Event, player_item_moved::PlayerItemMoved},
    utils::ids::parse_id,
};

pub fn handle_move_player_item(
    move_player_item: &MovePlayerItem,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Errors> {
    let item_id = parse_id(&move_player_item.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Errors::ItemNotFound(item_id.to_string())),
    };

    if character_item.is_weapon() && player.character.count_weapons_at_ready() >= 2 {
        return Err(Errors::TooManyWeaponsEquipped);
    }

    Ok(vec![Event::PlayerItemMoved(PlayerItemMoved {
        item_id,
        at_the_ready: move_player_item.put_at_the_ready,
        location: move_player_item.location_tag.clone(),
    })])
}
