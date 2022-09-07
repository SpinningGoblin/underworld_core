use rand::Rng;

use crate::{
    actions::MovePlayerItem,
    components::{
        items::{packed_tags_for_item_type, ready_tag_for_item_type},
        PlayerCharacter,
    },
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

    if character_item.is_at_the_ready() && move_player_item.put_at_the_ready {
        return Ok(Vec::new());
    }

    if character_item.is_packed() && !move_player_item.put_at_the_ready {
        return Ok(Vec::new());
    }

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

    let location = match &move_player_item.location_tag {
        Some(it) => *it,
        None => {
            if move_player_item.put_at_the_ready {
                ready_tag_for_item_type(&character_item.item.item_type)
            } else {
                let mut rng = rand::thread_rng();
                let possibilities = packed_tags_for_item_type(&character_item.item.item_type);
                let index = rng.gen_range(0..possibilities.len());
                possibilities.get(index).cloned().unwrap()
            }
        }
    };

    Ok(vec![Event::PlayerItemMoved(PlayerItemMoved {
        item_id,
        at_the_ready: move_player_item.put_at_the_ready,
        location,
    })])
}
