use crate::{
    actions::PickUpItem, components::games::GameState, errors::Error, events::Event,
    utils::parse_id,
};

pub fn handle(pick_up_item: &PickUpItem, state: &GameState) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&pick_up_item.item_id)?;

    match state
        .current_room()
        .loose_items
        .iter()
        .find(|item| item.id.eq(&item_id))
    {
        Some(_) => {}
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    Ok(vec![Event::PlayerPicksUpItem(item_id)])
}
