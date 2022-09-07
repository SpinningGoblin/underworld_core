#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::components::Tag;

use super::LocationTag;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
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
    pub fn matches_any_location_tags(&self, tags: &[LocationTag]) -> bool {
        tags.iter().any(|tag| self.tags().contains(tag))
    }

    pub fn matches_any_item_tags(&self, tags: &[Tag]) -> bool {
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
                vec![LocationTag::Hand]
            }
            LocationDescriptor::ClenchedInFist => vec![LocationTag::Hand],
            LocationDescriptor::DanglingFromWrists => vec![LocationTag::Wrist, LocationTag::Ankle],
            LocationDescriptor::HangingHip => vec![LocationTag::Hip],
            LocationDescriptor::HangingLooselyShoulders => {
                vec![LocationTag::Shoulder]
            }
            LocationDescriptor::HeldLoosely => vec![LocationTag::Hand],
            LocationDescriptor::HangingMoldySheath => {
                vec![LocationTag::HipSheath]
            }
            LocationDescriptor::SheathedAtHip => {
                vec![LocationTag::HipSheath]
            }
            LocationDescriptor::StrappedToBack => vec![LocationTag::Back],
            LocationDescriptor::StrappedToThigh => vec![LocationTag::Leg],
            LocationDescriptor::ClenchedInFists => vec![LocationTag::Hand],
            LocationDescriptor::None => Vec::new(),
            LocationDescriptor::CoiledAtWaist => vec![LocationTag::Waist],
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
