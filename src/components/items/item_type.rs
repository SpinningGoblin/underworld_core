#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::components::tag::{Tag, Tagged};

#[derive(Clone, Debug, EnumIter, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum ItemType {
    Breastplate,
    Boots,
    BowlerHat,
    Buckler,
    Cloak,
    Club,
    Crown,
    Dagger,
    Dirk,
    Fedora,
    Flask,
    Gloves,
    GreatSword,
    Halberd,
    Hammer,
    Helm,
    LoinCloth,
    LongSword,
    Mace,
    Mask,
    Morningstar,
    Pike,
    PlateBoots,
    PlateGauntlets,
    PlateHelmet,
    Pot,
    Scroll,
    Shield,
    ShortSword,
    Shirt,
    Shackles,
    Spear,
    TopHat,
    Trousers,
    Vest,
    Whip,
}

impl ItemType {
    pub fn is_lower_body(&self) -> bool {
        matches!(*self, ItemType::LoinCloth | ItemType::Trousers)
    }

    pub fn is_headgear(&self) -> bool {
        matches!(
            *self,
            ItemType::Crown | ItemType::Mask | ItemType::PlateHelmet
        )
    }

    pub fn is_upper_body(&self) -> bool {
        matches!(
            *self,
            ItemType::Breastplate | ItemType::Shirt | ItemType::Vest
        )
    }

    pub fn is_footwear(&self) -> bool {
        matches!(*self, ItemType::Boots | ItemType::PlateBoots)
    }

    pub fn is_for_hands(&self) -> bool {
        matches!(
            *self,
            ItemType::Shackles | ItemType::Boots | ItemType::PlateBoots
        )
    }
}

impl Tagged for ItemType {
    fn tags(&self) -> Vec<Tag> {
        match *self {
            ItemType::Breastplate => vec![Tag::Armour, Tag::Defense],
            ItemType::Boots => vec![Tag::Armour, Tag::Defense],
            ItemType::Buckler => vec![Tag::Shield, Tag::Defense, Tag::Damage],
            ItemType::Cloak => vec![Tag::Clothing],
            ItemType::Club => vec![Tag::Blunt, Tag::Damage],
            ItemType::Crown => vec![Tag::Accessory],
            ItemType::Dagger => vec![Tag::Blade, Tag::Damage],
            ItemType::Dirk => vec![Tag::Blade, Tag::Damage],
            ItemType::Gloves => vec![Tag::Armour, Tag::Defense],
            ItemType::GreatSword => vec![Tag::Blade, Tag::Damage],
            ItemType::Hammer => vec![Tag::Blunt, Tag::Damage],
            ItemType::Helm => vec![Tag::Armour, Tag::Defense],
            ItemType::LoinCloth => vec![Tag::Clothing],
            ItemType::LongSword => vec![Tag::Blade, Tag::Damage],
            ItemType::Mace => vec![Tag::Blunt, Tag::Damage],
            ItemType::Mask => vec![Tag::Accessory],
            ItemType::Morningstar => vec![Tag::Blunt, Tag::Damage],
            ItemType::PlateBoots => vec![Tag::Armour, Tag::Defense],
            ItemType::PlateGauntlets => vec![Tag::Armour, Tag::Defense],
            ItemType::PlateHelmet => vec![Tag::Armour, Tag::Defense],
            ItemType::Shield => vec![Tag::Shield, Tag::Damage, Tag::Defense],
            ItemType::ShortSword => vec![Tag::Blade, Tag::Damage],
            ItemType::Shirt => vec![Tag::Clothing],
            ItemType::Shackles => vec![Tag::Accessory],
            ItemType::Trousers => vec![Tag::Clothing],
            ItemType::Vest => vec![Tag::Clothing],
            ItemType::Whip => vec![Tag::Whip, Tag::Rope, Tag::Damage],
            ItemType::Halberd => vec![Tag::Damage],
            ItemType::Pike => vec![Tag::Damage],
            ItemType::Spear => vec![Tag::Damage],
            ItemType::BowlerHat => vec![Tag::Clothing],
            ItemType::Fedora => vec![Tag::Clothing],
            ItemType::TopHat => vec![Tag::Clothing],
            ItemType::Scroll => vec![Tag::Consumable, Tag::Teachable],
            ItemType::Pot => vec![Tag::Consumable, Tag::Throwable],
            ItemType::Flask => vec![Tag::Consumable],
        }
    }
}
