#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use super::{
    defense::Defense,
    descriptor_tags::{DescriptorTag, DescriptorTagged},
    equipped_item::{Equippable, EquippedLocation},
    item_descriptor::ItemDescriptor,
    item_material::{BuiltWithMaterial, ItemMaterial},
};

#[derive(Clone, Debug, PartialEq)]
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
        vec![
            WearableType::Breastplate,
            WearableType::Boots,
            WearableType::Cloak,
            WearableType::Crown,
            WearableType::Gloves,
            WearableType::LoinCloth,
            WearableType::Mask,
            WearableType::PlateBoots,
            WearableType::PlateGauntlets,
            WearableType::PlateHelmet,
            WearableType::Shackles,
            WearableType::Shirt,
            WearableType::Trousers,
            WearableType::Vest,
        ]
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
    fn possible_materials(&self) -> Vec<ItemMaterial> {
        match *self {
            WearableType::Breastplate => vec![
                ItemMaterial::Iron,
                ItemMaterial::Leather,
                ItemMaterial::Steel,
            ],
            WearableType::Mask => vec![ItemMaterial::Bone, ItemMaterial::Cloth, ItemMaterial::Iron],
            WearableType::Cloak => vec![ItemMaterial::Cloth],
            WearableType::Shirt => vec![ItemMaterial::Cloth],
            WearableType::Trousers => vec![
                ItemMaterial::Cloth,
                ItemMaterial::Hide,
                ItemMaterial::Leather,
            ],
            WearableType::Crown => {
                vec![ItemMaterial::Bone, ItemMaterial::Gold, ItemMaterial::Stone]
            }
            WearableType::Boots => vec![
                ItemMaterial::Hide,
                ItemMaterial::Iron,
                ItemMaterial::Leather,
                ItemMaterial::Steel,
            ],
            WearableType::Gloves => vec![ItemMaterial::Hide, ItemMaterial::Leather],
            WearableType::LoinCloth => vec![ItemMaterial::Hide, ItemMaterial::Cloth],
            WearableType::PlateBoots => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WearableType::PlateGauntlets => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WearableType::PlateHelmet => vec![ItemMaterial::Iron, ItemMaterial::Steel],
            WearableType::Shackles => vec![
                ItemMaterial::Iron,
                ItemMaterial::Leather,
                ItemMaterial::Steel,
            ],
            WearableType::Vest => vec![
                ItemMaterial::Cloth,
                ItemMaterial::Hide,
                ItemMaterial::Leather,
            ],
        }
    }
}

impl DescriptorTagged for WearableType {
    fn descriptor_tag(&self) -> DescriptorTag {
        match *self {
            WearableType::Breastplate => DescriptorTag::Armour,
            WearableType::Mask => DescriptorTag::Accessory,
            WearableType::Cloak => DescriptorTag::Clothing,
            WearableType::Shirt => DescriptorTag::Clothing,
            WearableType::Trousers => DescriptorTag::Clothing,
            WearableType::Crown => DescriptorTag::Accessory,
            WearableType::Boots => DescriptorTag::Armour,
            WearableType::Gloves => DescriptorTag::Clothing,
            WearableType::LoinCloth => DescriptorTag::Clothing,
            WearableType::PlateBoots => DescriptorTag::Armour,
            WearableType::PlateGauntlets => DescriptorTag::Armour,
            WearableType::PlateHelmet => DescriptorTag::Armour,
            WearableType::Shackles => DescriptorTag::Accessory,
            WearableType::Vest => DescriptorTag::Clothing,
        }
    }
}

impl Equippable for WearableType {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation> {
        match *self {
            WearableType::Breastplate => Vec::new(),
            WearableType::Cloak => vec![EquippedLocation::HangingLooselyShoulders],
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub struct Wearable {
    pub wearable_type: WearableType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<ItemMaterial>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<ItemDescriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub defense: Option<Defense>,
}

impl Equippable for Wearable {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation> {
        self.wearable_type.possible_equip_locations()
    }

    fn is_multiple(&self) -> bool {
        self.wearable_type.is_multiple()
    }
}

impl Display for Wearable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();

        self.descriptors
            .iter()
            .for_each(|quality| descriptions.push(quality.to_string()));

        if let Some(material) = &self.material {
            descriptions.push(material.to_string());
        }

        descriptions.push(self.wearable_type.to_string());
        write!(f, "{}", descriptions.join(" "))
    }
}

#[cfg(test)]
mod wearable_type_tests {
    use super::WearableType;

    #[test]
    fn display() {
        assert_eq!("armour", format!("{}", WearableType::Breastplate));
        assert_eq!("cloak", format!("{}", WearableType::Cloak));
        assert_eq!("shirt", format!("{}", WearableType::Shirt));
        assert_eq!("plate helmet", format!("{}", WearableType::PlateHelmet));
        assert_eq!("shackles", format!("{}", WearableType::Shackles));
    }

    #[test]
    fn unable_to_be_used_with() {
        let wearable_type = WearableType::Shirt;
        assert!(wearable_type.unable_to_be_used_with(&WearableType::Vest));
    }
}

#[cfg(test)]
mod wearable_quality_tests {
    use crate::components::item_descriptor::ItemDescriptor;

    #[test]
    fn display() {
        assert_eq!("bloodstained", format!("{}", ItemDescriptor::Bloodstained));
        assert_eq!("broken", format!("{}", ItemDescriptor::Broken));
        assert_eq!("colourful", format!("{}", ItemDescriptor::Colourful));
        assert_eq!("dingy", format!("{}", ItemDescriptor::Dingy));
        assert_eq!("drab", format!("{}", ItemDescriptor::Drab));
        assert_eq!("ill fitting", format!("{}", ItemDescriptor::IllFitting));
        assert_eq!("loose fitting", format!("{}", ItemDescriptor::LooseFitting));
        assert_eq!("rusty", format!("{}", ItemDescriptor::Rusty));
        assert_eq!("shimmering", format!("{}", ItemDescriptor::Shimmering));
        assert_eq!("shiny", format!("{}", ItemDescriptor::Shiny));
        assert_eq!("stained", format!("{}", ItemDescriptor::Stained));
    }
}

#[cfg(test)]
mod wearable_tests {
    use crate::components::item_material::ItemMaterial;

    use super::{ItemDescriptor, Wearable, WearableType};

    #[test]
    fn display_when_there_is_only_type() {
        let wearable = Wearable {
            wearable_type: WearableType::Breastplate,
            material: None,
            descriptors: Vec::new(),
            defense: None,
        };

        assert_eq!("armour", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_is_material() {
        let wearable = Wearable {
            wearable_type: WearableType::PlateHelmet,
            material: Some(ItemMaterial::Steel),
            descriptors: Vec::new(),
            defense: None,
        };

        assert_eq!("steel plate helmet", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: None,
            descriptors: vec![ItemDescriptor::Dingy, ItemDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("dingy bloodstained shackles", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities_and_material() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(ItemMaterial::Iron),
            descriptors: vec![ItemDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("bloodstained iron shackles", format!("{}", wearable));
    }
}
