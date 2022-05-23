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
        | ItemType::Spear
        | ItemType::TopHat
        | ItemType::BowlerHat
        | ItemType::Fedora
        | ItemType::Scroll => false,
        ItemType::Trousers
        | ItemType::Breastplate
        | ItemType::Boots
        | ItemType::Gloves
        | ItemType::PlateBoots
        | ItemType::PlateGauntlets => true,
    }
}
