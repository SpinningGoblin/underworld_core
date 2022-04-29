use enum_iterator::IntoEnumIterator;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::item_type::ItemType;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
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

    pub fn weapon_tags() -> Vec<LocationTag> {
        vec![
            LocationTag::Hand,
            LocationTag::Hip,
            LocationTag::HipSheath,
            LocationTag::Back,
        ]
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
        ItemType::Crown => vec![LocationTag::Head],
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
    }
}
