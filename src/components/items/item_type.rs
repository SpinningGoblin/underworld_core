#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::components::tag::{Tag, Tagged};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
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
            ItemType::Breastplate => {
                vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable]
            }
            ItemType::Boots => vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable],
            ItemType::Buckler => vec![Tag::Shield, Tag::Defense, Tag::Damage, Tag::Equippable],
            ItemType::Cloak => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Club => vec![Tag::Blunt, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Crown => vec![Tag::Accessory, Tag::Wearable, Tag::Equippable],
            ItemType::Dagger => vec![Tag::Blade, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Dirk => vec![Tag::Blade, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Gloves => vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable],
            ItemType::GreatSword => vec![Tag::Blade, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Hammer => vec![Tag::Blunt, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Helm => vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable],
            ItemType::LoinCloth => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::LongSword => vec![Tag::Blade, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Mace => vec![Tag::Blunt, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Mask => vec![Tag::Accessory, Tag::Wearable, Tag::Equippable],
            ItemType::Morningstar => vec![Tag::Blunt, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::PlateBoots => vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable],
            ItemType::PlateGauntlets => {
                vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable]
            }
            ItemType::PlateHelmet => {
                vec![Tag::Armour, Tag::Defense, Tag::Wearable, Tag::Equippable]
            }
            ItemType::Shield => vec![Tag::Shield, Tag::Damage, Tag::Defense, Tag::Equippable],
            ItemType::ShortSword => vec![Tag::Blade, Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Shirt => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Shackles => vec![Tag::Accessory, Tag::Wearable, Tag::Equippable],
            ItemType::Trousers => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Vest => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Whip => vec![
                Tag::Whip,
                Tag::Rope,
                Tag::Damage,
                Tag::Weapon,
                Tag::Equippable,
            ],
            ItemType::Halberd => vec![Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Pike => vec![Tag::Damage, Tag::Weapon, Tag::Equippable],
            ItemType::Spear => vec![Tag::Damage, Tag::Weapon, Tag::Thrust, Tag::Equippable],
            ItemType::BowlerHat => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Fedora => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::TopHat => vec![Tag::Clothing, Tag::Wearable, Tag::Equippable],
            ItemType::Scroll => vec![Tag::Consumable, Tag::Teachable],
            ItemType::Pot => vec![Tag::Consumable, Tag::Throwable],
            ItemType::Flask => vec![Tag::Consumable],
        }
    }
}
