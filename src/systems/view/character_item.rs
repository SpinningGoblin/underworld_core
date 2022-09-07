use crate::components::items::{CharacterItem, CharacterItemView};

pub fn view(character_item: &CharacterItem, knows_all: bool) -> CharacterItemView {
    let full_item_hidden = character_item.equipped_location.hides_full_item();

    CharacterItemView {
        item: super::item::view(&character_item.item, !full_item_hidden, knows_all),
        at_the_ready: character_item.at_the_ready,
        equipped_location: character_item.equipped_location,
    }
}
