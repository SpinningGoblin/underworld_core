use crate::components::items::character_item::{CharacterItem, CharacterItemView};

pub fn look_at(
    character_item: &CharacterItem,
    knows_hidden: bool,
    knows_all: bool,
) -> CharacterItemView {
    let full_item_hidden = character_item
        .equipped_location_tags
        .iter()
        .any(|tag| tag.hides_full_item());

    let is_hidden = if knows_hidden || knows_all {
        Some(character_item.is_hidden)
    } else {
        None
    };

    let (location_descriptor, knows_equipped_location, equipped_location_tags) =
        if character_item.is_hidden && (!knows_hidden || knows_all) {
            (None, false, Vec::new())
        } else {
            (
                Some(character_item.location_descriptor.clone()),
                true,
                character_item.equipped_location_tags.clone(),
            )
        };

    CharacterItemView {
        item: super::item::look_at(&character_item.item, !full_item_hidden, knows_all),
        is_hidden,
        location_descriptor,
        knows_equipped_location,
        equipped_location_tags,
        is_multiple: character_item.is_multiple,
    }
}
