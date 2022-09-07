#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::item_type::ItemType;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "openapi",
    derive(Enum),
    oai(rename_all = "snake_case", rename = "EquipLocationTag")
)]
pub enum LocationTag {
    Ankle,
    Arm,
    Back,
    Body,
    Face,
    Feet,
    Hand,
    Head,
    Hip,
    HipSheath,
    Leg,
    Neck,
    Packed,
    Pockets,
    Shoulder,
    Waist,
    Wrist,
}

impl LocationTag {
    pub fn hides_full_item(&self) -> bool {
        matches!(
            *self,
            LocationTag::HipSheath | LocationTag::Packed | LocationTag::Pockets
        )
    }

    pub fn wearable_tags() -> Vec<LocationTag> {
        vec![
            LocationTag::Ankle,
            LocationTag::Arm,
            LocationTag::Back,
            LocationTag::Body,
            LocationTag::Feet,
            LocationTag::Hand,
            LocationTag::Head,
            LocationTag::Leg,
            LocationTag::Neck,
            LocationTag::Shoulder,
            LocationTag::Waist,
            LocationTag::Wrist,
        ]
    }

    pub fn not_at_ready_weapon_tags() -> Vec<LocationTag> {
        vec![LocationTag::Hip, LocationTag::HipSheath, LocationTag::Back]
    }
}

pub fn ready_tag_for_item_type(item_type: &ItemType) -> LocationTag {
    match *item_type {
        ItemType::Buckler
        | ItemType::Club
        | ItemType::Dagger
        | ItemType::Dirk
        | ItemType::GreatSword
        | ItemType::Hammer
        | ItemType::LongSword
        | ItemType::Mace
        | ItemType::Morningstar
        | ItemType::Shield
        | ItemType::ShortSword
        | ItemType::Whip
        | ItemType::Gloves
        | ItemType::PlateGauntlets
        | ItemType::Halberd
        | ItemType::Pike
        | ItemType::Spear => LocationTag::Hand,
        ItemType::Breastplate | ItemType::Shirt | ItemType::Vest => LocationTag::Body,
        ItemType::Mask
        | ItemType::Crown
        | ItemType::BowlerHat
        | ItemType::Fedora
        | ItemType::TopHat
        | ItemType::PlateHelmet
        | ItemType::Helm => LocationTag::Head,
        ItemType::Boots | ItemType::PlateBoots => LocationTag::Feet,
        ItemType::Trousers => LocationTag::Leg,
        ItemType::Cloak => LocationTag::Shoulder,
        ItemType::LoinCloth => LocationTag::Waist,
        ItemType::Shackles => LocationTag::Wrist,
        ItemType::Scroll => LocationTag::Pockets,
        ItemType::Pot => LocationTag::Packed,
        ItemType::Flask => LocationTag::Packed,
    }
}

pub fn packed_tags_for_item_type(item_type: &ItemType) -> Vec<LocationTag> {
    match *item_type {
        ItemType::Buckler => vec![LocationTag::Packed],
        ItemType::Club => vec![LocationTag::Packed, LocationTag::Hip],
        ItemType::Dagger => vec![
            LocationTag::Packed,
            LocationTag::Hip,
            LocationTag::HipSheath,
        ],
        ItemType::Dirk => vec![
            LocationTag::Packed,
            LocationTag::Hip,
            LocationTag::HipSheath,
        ],
        ItemType::GreatSword => vec![LocationTag::Packed, LocationTag::Back],
        ItemType::Hammer => vec![LocationTag::Packed, LocationTag::Hip],
        ItemType::LongSword => vec![
            LocationTag::Packed,
            LocationTag::Hip,
            LocationTag::HipSheath,
            LocationTag::Back,
        ],
        ItemType::Mace => vec![LocationTag::Packed, LocationTag::Hip],
        ItemType::Morningstar => vec![LocationTag::Packed, LocationTag::Hip],
        ItemType::Shield => vec![LocationTag::Packed, LocationTag::Back],
        ItemType::ShortSword => {
            vec![
                LocationTag::Packed,
                LocationTag::Hip,
                LocationTag::HipSheath,
            ]
        }
        ItemType::Whip => vec![LocationTag::Packed, LocationTag::Hip],
        ItemType::Breastplate => vec![LocationTag::Packed],
        ItemType::Mask => vec![LocationTag::Packed],
        ItemType::Cloak => vec![LocationTag::Packed],
        ItemType::Shirt => vec![LocationTag::Packed],
        ItemType::Trousers => vec![LocationTag::Packed],
        ItemType::Crown | ItemType::BowlerHat | ItemType::Fedora | ItemType::TopHat => {
            vec![LocationTag::Packed]
        }
        ItemType::Boots => vec![LocationTag::Packed],
        ItemType::Gloves => vec![LocationTag::Packed, LocationTag::Pockets],
        ItemType::LoinCloth => vec![LocationTag::Packed],
        ItemType::PlateBoots => vec![LocationTag::Packed],
        ItemType::PlateGauntlets => vec![LocationTag::Packed],
        ItemType::PlateHelmet => vec![LocationTag::Packed],
        ItemType::Shackles => vec![LocationTag::Packed],
        ItemType::Vest => vec![LocationTag::Packed],
        ItemType::Helm => vec![LocationTag::Packed],
        ItemType::Halberd => vec![LocationTag::Packed, LocationTag::Back],
        ItemType::Pike => vec![LocationTag::Packed, LocationTag::Back],
        ItemType::Spear => vec![LocationTag::Packed, LocationTag::Back],
        ItemType::Scroll => vec![LocationTag::Packed, LocationTag::Pockets],
        ItemType::Pot => vec![LocationTag::Packed],
        ItemType::Flask => vec![LocationTag::Packed],
    }
}

pub fn location_tags_for_item_type(item_type: &ItemType) -> Vec<LocationTag> {
    match *item_type {
        ItemType::Buckler => vec![LocationTag::Hand],
        ItemType::Club => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Dagger => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
        ItemType::Dirk => vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath],
        ItemType::GreatSword => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::Hammer => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::LongSword => vec![
            LocationTag::Hand,
            LocationTag::Hip,
            LocationTag::HipSheath,
            LocationTag::Back,
        ],
        ItemType::Mace => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Morningstar => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Shield => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::ShortSword => {
            vec![LocationTag::Hand, LocationTag::Hip, LocationTag::HipSheath]
        }
        ItemType::Whip => vec![LocationTag::Hand, LocationTag::Hip],
        ItemType::Breastplate => vec![LocationTag::Body],
        ItemType::Mask => vec![LocationTag::Head],
        ItemType::Cloak => vec![LocationTag::Shoulder],
        ItemType::Shirt => vec![LocationTag::Body],
        ItemType::Trousers => vec![LocationTag::Leg],
        ItemType::Crown | ItemType::BowlerHat | ItemType::Fedora | ItemType::TopHat => {
            vec![LocationTag::Head]
        }
        ItemType::Boots => vec![LocationTag::Feet],
        ItemType::Gloves => vec![LocationTag::Hand],
        ItemType::LoinCloth => vec![LocationTag::Waist],
        ItemType::PlateBoots => vec![LocationTag::Feet],
        ItemType::PlateGauntlets => vec![LocationTag::Hand],
        ItemType::PlateHelmet => vec![LocationTag::Head],
        ItemType::Shackles => vec![LocationTag::Ankle, LocationTag::Wrist],
        ItemType::Vest => vec![LocationTag::Body],
        ItemType::Helm => vec![LocationTag::Head],
        ItemType::Halberd => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::Pike => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::Spear => vec![LocationTag::Hand, LocationTag::Back],
        ItemType::Scroll => vec![LocationTag::Packed, LocationTag::Pockets],
        ItemType::Pot => vec![LocationTag::Packed],
        ItemType::Flask => vec![LocationTag::Packed],
    }
}
