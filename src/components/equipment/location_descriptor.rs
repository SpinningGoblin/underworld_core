use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::tag::Tag;

use super::location_tag::LocationTag;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum LocationDescriptor {
    AlmostFallingGrip,
    ClenchedInFist,
    CoiledAtWaist,
    DanglingFromWrists,
    HangingHip,
    HangingLooselyShoulders,
    HeldLoosely,
    HangingMoldySheath,
    SheathedAtHip,
    StrappedToBack,
    StrappedToThigh,
    ClenchedInFists,
    None,
}

impl Default for LocationDescriptor {
    fn default() -> Self {
        LocationDescriptor::None
    }
}

impl LocationDescriptor {
    pub fn matches_any_location_tags(&self, tags: Vec<LocationTag>) -> bool {
        tags.iter().any(|tag| self.tags().contains(tag))
    }

    pub fn matches_any_item_tags(&self, tags: Vec<Tag>) -> bool {
        tags.iter().any(|tag| self.item_tags().contains(tag))
    }

    pub fn item_tags(&self) -> Vec<Tag> {
        match *self {
            LocationDescriptor::AlmostFallingGrip => {
                vec![Tag::Blade, Tag::Blunt, Tag::Rope, Tag::Shield]
            }
            LocationDescriptor::ClenchedInFist => {
                vec![Tag::Blade, Tag::Blunt, Tag::Rope, Tag::Shield]
            }
            LocationDescriptor::DanglingFromWrists => vec![Tag::Rope],
            LocationDescriptor::HangingHip => vec![Tag::Blade, Tag::Blunt, Tag::Rope, Tag::Whip],
            LocationDescriptor::HangingLooselyShoulders => vec![Tag::Clothing],
            LocationDescriptor::HeldLoosely => {
                vec![Tag::Blade, Tag::Blunt, Tag::Rope, Tag::Whip, Tag::Shield]
            }
            LocationDescriptor::HangingMoldySheath => vec![Tag::Blade],
            LocationDescriptor::SheathedAtHip => vec![Tag::Blade],
            LocationDescriptor::StrappedToBack => {
                vec![Tag::Blade, Tag::Blunt, Tag::Shield]
            }
            LocationDescriptor::StrappedToThigh => vec![Tag::Blade],
            LocationDescriptor::ClenchedInFists => {
                vec![Tag::Blade, Tag::Blunt, Tag::Rope]
            }
            LocationDescriptor::None => Vec::new(),
            LocationDescriptor::CoiledAtWaist => vec![Tag::Rope, Tag::Whip],
        }
    }

    pub fn tags(&self) -> Vec<LocationTag> {
        match *self {
            LocationDescriptor::AlmostFallingGrip => {
                vec![LocationTag::Hand, LocationTag::Equipped]
            }
            LocationDescriptor::ClenchedInFist => vec![LocationTag::Hand, LocationTag::Equipped],
            LocationDescriptor::DanglingFromWrists => vec![
                LocationTag::Equipped,
                LocationTag::Wrist,
                LocationTag::Ankle,
            ],
            LocationDescriptor::HangingHip => vec![LocationTag::Equipped, LocationTag::Hip],
            LocationDescriptor::HangingLooselyShoulders => {
                vec![LocationTag::Equipped, LocationTag::Shoulder]
            }
            LocationDescriptor::HeldLoosely => vec![LocationTag::Equipped, LocationTag::Hand],
            LocationDescriptor::HangingMoldySheath => {
                vec![LocationTag::Equipped, LocationTag::HipSheath]
            }
            LocationDescriptor::SheathedAtHip => {
                vec![LocationTag::Equipped, LocationTag::HipSheath]
            }
            LocationDescriptor::StrappedToBack => vec![LocationTag::Equipped, LocationTag::Back],
            LocationDescriptor::StrappedToThigh => vec![LocationTag::Equipped, LocationTag::Leg],
            LocationDescriptor::ClenchedInFists => vec![LocationTag::Equipped, LocationTag::Hand],
            LocationDescriptor::None => Vec::new(),
            LocationDescriptor::CoiledAtWaist => vec![LocationTag::Equipped, LocationTag::Waist],
        }
    }

    pub fn unable_to_be_used_with(&self, other: &LocationDescriptor) -> bool {
        match *self {
            LocationDescriptor::AlmostFallingGrip => other.is_in_hand(),
            LocationDescriptor::ClenchedInFist => other.is_in_hand(),
            LocationDescriptor::DanglingFromWrists => false,
            LocationDescriptor::HangingHip => other.is_at_hip(),
            LocationDescriptor::HangingLooselyShoulders => false,
            LocationDescriptor::HeldLoosely => other.is_in_hand(),
            LocationDescriptor::HangingMoldySheath => other.is_at_hip(),
            LocationDescriptor::SheathedAtHip => other.is_at_hip(),
            LocationDescriptor::StrappedToBack => false,
            LocationDescriptor::StrappedToThigh => false,
            LocationDescriptor::ClenchedInFists => other.is_in_hand(),
            LocationDescriptor::None => false,
            LocationDescriptor::CoiledAtWaist => other.is_at_hip(),
        }
    }

    fn is_in_hand(&self) -> bool {
        matches!(
            *self,
            LocationDescriptor::AlmostFallingGrip
                | LocationDescriptor::ClenchedInFist
                | LocationDescriptor::ClenchedInFists
                | LocationDescriptor::HeldLoosely
        )
    }

    fn is_at_hip(&self) -> bool {
        matches!(
            *self,
            LocationDescriptor::HangingHip
                | LocationDescriptor::HangingMoldySheath
                | LocationDescriptor::SheathedAtHip
        )
    }
}

impl Display for LocationDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LocationDescriptor::StrappedToThigh => write!(f, "strapped to its thigh"),
            LocationDescriptor::ClenchedInFist => write!(f, "clenched in its fist"),
            LocationDescriptor::HeldLoosely => write!(f, "held loosely"),
            LocationDescriptor::StrappedToBack => write!(f, "strapped to its back"),
            LocationDescriptor::SheathedAtHip => write!(f, "sheathed at its hip"),
            LocationDescriptor::HangingMoldySheath => write!(f, "hanging in a moldy sheath"),
            LocationDescriptor::HangingLooselyShoulders => {
                write!(f, "which hangs loosely around its shoulders")
            }
            LocationDescriptor::DanglingFromWrists => write!(f, "dangling from its wrists"),
            LocationDescriptor::HangingHip => write!(f, "hanging at its hip"),
            LocationDescriptor::AlmostFallingGrip => write!(f, "almost falling from its grip"),
            LocationDescriptor::ClenchedInFists => write!(f, "clenched in its fists"),
            LocationDescriptor::None => write!(f, ""),
            LocationDescriptor::CoiledAtWaist => write!(f, "coiled at its waist"),
        }
    }
}
