use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    equipped::equip_location_descriptor::EquipLocationDescriptor,
    item::{EquippableItem, Item},
    item_descriptor::ItemDescriptor,
    item_tag::{ItemTag, TaggedItem},
    material::{BuiltWithMaterial, Material},
};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WearableType {
    Breastplate,
    Mask,
    Cloak,
    Shirt,
    Trousers,
    Crown,
    Boots,
    Gloves,
    LoinCloth,
    PlateBoots,
    PlateGauntlets,
    PlateHelmet,
    Shackles,
    Vest,
}

impl WearableType {
    pub fn all() -> Vec<WearableType> {
        WearableType::into_enum_iter().collect()
    }

    pub fn unable_to_be_used_with(&self, other: &WearableType) -> bool {
        match *self {
            WearableType::Breastplate => other.is_upper_body(),
            WearableType::Boots => other.is_footwear(),
            WearableType::Cloak => other == &WearableType::Cloak,
            WearableType::Crown => other.is_headgear(),
            WearableType::Gloves => other.is_for_hands(),
            WearableType::LoinCloth => other.is_lower_body(),
            WearableType::Mask => other.is_headgear(),
            WearableType::PlateBoots => other.is_footwear(),
            WearableType::PlateGauntlets => other.is_footwear(),
            WearableType::PlateHelmet => other.is_headgear(),
            WearableType::Shackles => other.is_for_hands(),
            WearableType::Shirt => other.is_upper_body(),
            WearableType::Trousers => other.is_lower_body(),
            WearableType::Vest => other.is_upper_body(),
        }
    }

    pub fn is_lower_body(&self) -> bool {
        matches!(*self, WearableType::LoinCloth | WearableType::Trousers)
    }

    pub fn is_headgear(&self) -> bool {
        matches!(
            *self,
            WearableType::Crown | WearableType::Mask | WearableType::PlateHelmet
        )
    }

    pub fn is_upper_body(&self) -> bool {
        matches!(
            *self,
            WearableType::Breastplate | WearableType::Shirt | WearableType::Vest
        )
    }

    pub fn is_footwear(&self) -> bool {
        matches!(*self, WearableType::Boots | WearableType::PlateBoots)
    }

    pub fn is_for_hands(&self) -> bool {
        matches!(
            *self,
            WearableType::Shackles | WearableType::Boots | WearableType::PlateBoots
        )
    }

    pub fn necessary_descriptors(&self) -> Vec<ItemDescriptor> {
        match *self {
            WearableType::Breastplate => Vec::new(),
            WearableType::Cloak => Vec::new(),
            WearableType::Shirt => Vec::new(),
            WearableType::PlateHelmet => Vec::new(),
            WearableType::Shackles => vec![ItemDescriptor::SetOf],
            WearableType::Mask => Vec::new(),
            WearableType::Trousers => Vec::new(),
            WearableType::Crown => Vec::new(),
            WearableType::Boots => Vec::new(),
            WearableType::Gloves => Vec::new(),
            WearableType::LoinCloth => Vec::new(),
            WearableType::PlateBoots => Vec::new(),
            WearableType::PlateGauntlets => Vec::new(),
            WearableType::Vest => Vec::new(),
        }
    }
}

impl BuiltWithMaterial for WearableType {
    fn possible_materials(&self) -> Vec<Material> {
        match *self {
            WearableType::Breastplate => vec![Material::Iron, Material::Leather, Material::Steel],
            WearableType::Mask => vec![Material::Bone, Material::Iron],
            WearableType::Cloak => {
                vec![Material::Linen, Material::Hide, Material::Wool]
            }
            WearableType::Shirt => vec![
                Material::Wool,
                Material::Linen,
                Material::Cotton,
                Material::Silk,
            ],
            WearableType::Trousers => vec![
                Material::Hide,
                Material::Leather,
                Material::Wool,
                Material::Linen,
            ],
            WearableType::Crown => {
                vec![Material::Bone, Material::Gold, Material::Stone]
            }
            WearableType::Boots => vec![
                Material::Hide,
                Material::Iron,
                Material::Leather,
                Material::Steel,
            ],
            WearableType::Gloves => vec![Material::Hide, Material::Leather],
            WearableType::LoinCloth => vec![
                Material::Hide,
                Material::Wool,
                Material::Leather,
                Material::Silk,
                Material::Linen,
                Material::Cotton,
            ],
            WearableType::PlateBoots => vec![Material::Iron, Material::Steel],
            WearableType::PlateGauntlets => vec![Material::Iron, Material::Steel],
            WearableType::PlateHelmet => vec![Material::Iron, Material::Steel],
            WearableType::Shackles => vec![Material::Iron, Material::Leather, Material::Steel],
            WearableType::Vest => {
                vec![Material::Fur, Material::Hide, Material::Leather]
            }
        }
    }
}

impl TaggedItem for WearableType {
    fn tags(&self) -> Vec<ItemTag> {
        match *self {
            WearableType::Breastplate => vec![ItemTag::Armour],
            WearableType::Mask => vec![ItemTag::Accessory],
            WearableType::Cloak => vec![ItemTag::Clothing],
            WearableType::Shirt => vec![ItemTag::Clothing],
            WearableType::Trousers => vec![ItemTag::Clothing],
            WearableType::Crown => vec![ItemTag::Accessory],
            WearableType::Boots => vec![ItemTag::Armour],
            WearableType::Gloves => vec![ItemTag::Clothing],
            WearableType::LoinCloth => vec![ItemTag::Clothing],
            WearableType::PlateBoots => vec![ItemTag::Armour],
            WearableType::PlateGauntlets => vec![ItemTag::Armour],
            WearableType::PlateHelmet => vec![ItemTag::Armour],
            WearableType::Shackles => vec![ItemTag::Accessory],
            WearableType::Vest => vec![ItemTag::Clothing],
        }
    }
}

impl EquippableItem for WearableType {
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor> {
        match *self {
            WearableType::Breastplate => Vec::new(),
            WearableType::Cloak => vec![EquipLocationDescriptor::HangingLooselyShoulders],
            WearableType::Shirt => Vec::new(),
            WearableType::PlateHelmet => Vec::new(),
            WearableType::Shackles => Vec::new(),
            WearableType::Mask => Vec::new(),
            WearableType::Trousers => Vec::new(),
            WearableType::Crown => Vec::new(),
            WearableType::Boots => Vec::new(),
            WearableType::Gloves => Vec::new(),
            WearableType::LoinCloth => Vec::new(),
            WearableType::PlateBoots => Vec::new(),
            WearableType::PlateGauntlets => Vec::new(),
            WearableType::Vest => Vec::new(),
        }
    }
}

impl Item for WearableType {
    fn look_at(&self, _is_equipped: bool) -> String {
        format!("{}", self)
    }

    fn is_multiple(&self) -> bool {
        match *self {
            WearableType::Breastplate => true,
            WearableType::Cloak => false,
            WearableType::Shirt => false,
            WearableType::PlateHelmet => false,
            WearableType::Shackles => false,
            WearableType::Mask => false,
            WearableType::Trousers => true,
            WearableType::Crown => false,
            WearableType::Boots => true,
            WearableType::Gloves => true,
            WearableType::LoinCloth => false,
            WearableType::PlateBoots => true,
            WearableType::PlateGauntlets => true,
            WearableType::Vest => false,
        }
    }

    fn material(&self) -> Option<Material> {
        None
    }
}

impl Display for WearableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WearableType::Breastplate => write!(f, "armour"),
            WearableType::Cloak => write!(f, "cloak"),
            WearableType::Shirt => write!(f, "shirt"),
            WearableType::PlateHelmet => write!(f, "plate helmet"),
            WearableType::Shackles => write!(f, "shackles"),
            WearableType::Mask => write!(f, "mask"),
            WearableType::Trousers => write!(f, "trousers"),
            WearableType::Crown => write!(f, "crown"),
            WearableType::Boots => write!(f, "boots"),
            WearableType::Gloves => write!(f, "gloves"),
            WearableType::LoinCloth => write!(f, "loin cloth"),
            WearableType::PlateBoots => write!(f, "plate boots"),
            WearableType::PlateGauntlets => write!(f, "plate gauntlets"),
            WearableType::Vest => write!(f, "vest"),
        }
    }
}
