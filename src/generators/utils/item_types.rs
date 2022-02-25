use crate::components::{items::item_type::ItemType, tag::Tagged};

pub fn type_is_for_weapon(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_weapon())
}

pub fn type_is_for_wearable(item_type: &ItemType) -> bool {
    item_type.tags().iter().any(|tag| tag.is_wearable())
}

pub fn type_inherently_multiple(item_type: &ItemType) -> bool {
    match *item_type {
        ItemType::Club
        | ItemType::Dagger
        | ItemType::Hammer
        | ItemType::LongSword
        | ItemType::ShortSword
        | ItemType::Buckler
        | ItemType::Dirk
        | ItemType::GreatSword
        | ItemType::Mace
        | ItemType::Morningstar
        | ItemType::Shield
        | ItemType::Whip
        | ItemType::Cloak
        | ItemType::Shirt
        | ItemType::PlateHelmet
        | ItemType::Shackles
        | ItemType::Mask
        | ItemType::Crown
        | ItemType::LoinCloth
        | ItemType::Vest
        | ItemType::Helm
        | ItemType::Halberd
        | ItemType::Pike
        | ItemType::Spear => false,
        ItemType::Trousers
        | ItemType::Breastplate
        | ItemType::Boots
        | ItemType::Gloves
        | ItemType::PlateBoots
        | ItemType::PlateGauntlets => true,
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
