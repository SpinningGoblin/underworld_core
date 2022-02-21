use crate::components::{items::item_type::ItemType, tag::Tagged};

pub fn type_is_for_weapon(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_weapon())
}

pub fn type_is_for_wearable(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_wearable())
}

pub fn type_inherently_multiple(item_type: &ItemType) -> bool {
    match *item_type {
        ItemType::Club => false,
        ItemType::Dagger => false,
        ItemType::Hammer => false,
        ItemType::LongSword => false,
        ItemType::ShortSword => false,
        ItemType::Buckler => false,
        ItemType::Dirk => false,
        ItemType::GreatSword => false,
        ItemType::Mace => false,
        ItemType::Morningstar => false,
        ItemType::Shield => false,
        ItemType::Whip => false,
        ItemType::Breastplate => true,
        ItemType::Cloak => false,
        ItemType::Shirt => false,
        ItemType::PlateHelmet => false,
        ItemType::Shackles => false,
        ItemType::Mask => false,
        ItemType::Trousers => true,
        ItemType::Crown => false,
        ItemType::Boots => true,
        ItemType::Gloves => true,
        ItemType::LoinCloth => false,
        ItemType::PlateBoots => true,
        ItemType::PlateGauntlets => true,
        ItemType::Vest => false,
        ItemType::Helm => false,
    }
}

pub fn type_cannot_be_used_with(item_type: &ItemType, other: &ItemType) -> bool {
    match *item_type {
        ItemType::Breastplate => other.is_upper_body(),
        ItemType::Boots => other.is_footwear(),
        ItemType::Cloak => other == &ItemType::Cloak,
        ItemType::Crown => other.is_headgear(),
        ItemType::Gloves => other.is_for_hands(),
        ItemType::LoinCloth => other.is_lower_body(),
        ItemType::Mask => other.is_headgear(),
        ItemType::PlateBoots => other.is_footwear(),
        ItemType::PlateGauntlets => other.is_footwear(),
        ItemType::PlateHelmet => other.is_headgear(),
        ItemType::Shackles => other.is_for_hands(),
        ItemType::Shirt => other.is_upper_body(),
        ItemType::Trousers => other.is_lower_body(),
        ItemType::Vest => other.is_upper_body(),
        _ => false,
    }
}
