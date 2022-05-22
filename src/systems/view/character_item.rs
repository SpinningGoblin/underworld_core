use crate::components::items::character_item::{CharacterItem, CharacterItemView};

pub fn view(
    character_item: &CharacterItem,
    knows_hidden: bool,
    knows_all: bool,
) -> CharacterItemView {
    let full_item_hidden = character_item.equipped_location.hides_full_item();

    let is_hidden = if knows_hidden || knows_all {
        Some(character_item.is_hidden)
    } else {
        None
    };

    let (knows_equipped_location, equipped_location) =
        if character_item.is_hidden && (!knows_hidden || knows_all) {
            (false, None)
        } else {
            (true, Some(character_item.equipped_location.clone()))
        };

    CharacterItemView {
        item: super::item::view(&character_item.item, !full_item_hidden, knows_all),
        is_hidden,
        knows_equipped_location,
        is_multiple: character_item.is_multiple,
        at_the_ready: character_item.at_the_ready,
        equipped_location,
    }
}
