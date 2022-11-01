use crate::components::{items::ItemType, Tagged};

pub fn type_is_for_weapon(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_weapon())
}

pub fn type_is_for_wearable(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_wearable())
}
