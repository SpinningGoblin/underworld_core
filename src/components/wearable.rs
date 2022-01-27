#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use super::{
    defense::Defense,
    equipped_item::{Equippable, EquippedLocation},
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

    pub fn necessary_descriptors(&self) -> Vec<WearableDescriptor> {
        match *self {
            WearableType::Breastplate => Vec::new(),
            WearableType::Cloak => Vec::new(),
            WearableType::Shirt => Vec::new(),
            WearableType::PlateHelmet => Vec::new(),
            WearableType::Shackles => vec![WearableDescriptor::SetOf],
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

    pub fn possible_descriptors(&self) -> Vec<WearableDescriptor> {
        match *self {
            WearableType::Breastplate => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Rusty,
                WearableDescriptor::Stained,
            ],
            WearableType::Cloak => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Shirt => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::PlateHelmet => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Shackles => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Mask => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Stained,
            ],
            WearableType::Trousers => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Crown => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
            ],
            WearableType::Boots => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Gloves => vec![
                WearableDescriptor::Dingy,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Stained,
            ],
            WearableType::LoinCloth => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::PlateBoots => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::PlateGauntlets => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::Vest => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
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
            Self::Breastplate => write!(f, "armour"),
            Self::Cloak => write!(f, "cloak"),
            Self::Shirt => write!(f, "shirt"),
            Self::PlateHelmet => write!(f, "plate helmet"),
            Self::Shackles => write!(f, "shackles"),
            Self::Mask => write!(f, "mask"),
            Self::Trousers => write!(f, "trousers"),
            Self::Crown => write!(f, "crown"),
            Self::Boots => write!(f, "boots"),
            Self::Gloves => write!(f, "gloves"),
            Self::LoinCloth => write!(f, "loin cloth"),
            Self::PlateBoots => write!(f, "plate boots"),
            Self::PlateGauntlets => write!(f, "plate gauntlets"),
            Self::Vest => write!(f, "vest"),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WearableDescriptor {
    Bloodstained,
    Broken,
    Colourful,
    Dingy,
    Drab,
    IllFitting,
    LooseFitting,
    Rusty,
    SetOf,
    Shimmering,
    Shiny,
    Stained,
}

impl Display for WearableDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bloodstained => write!(f, "bloodstained"),
            Self::Broken => write!(f, "broken"),
            Self::Colourful => write!(f, "colourful"),
            Self::Dingy => write!(f, "dingy"),
            Self::Drab => write!(f, "drab"),
            Self::IllFitting => write!(f, "ill fitting"),
            Self::LooseFitting => write!(f, "loose fitting"),
            Self::Rusty => write!(f, "rusty"),
            Self::SetOf => write!(f, "set of"),
            Self::Shimmering => write!(f, "shimmering"),
            Self::Shiny => write!(f, "shiny"),
            Self::Stained => write!(f, "stained"),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WearableMaterial {
    Bone,
    Cloth,
    Gold,
    Iron,
    Leather,
    Steel,
}

impl Display for WearableMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WearableMaterial::Bone => write!(f, "bone"),
            WearableMaterial::Cloth => write!(f, "cloth"),
            WearableMaterial::Iron => write!(f, "iron"),
            WearableMaterial::Leather => write!(f, "leather"),
            WearableMaterial::Steel => write!(f, "steel"),
            WearableMaterial::Gold => write!(f, "gold"),
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
    pub material: Option<WearableMaterial>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<WearableDescriptor>,
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
    use super::WearableDescriptor;

    #[test]
    fn display() {
        assert_eq!(
            "bloodstained",
            format!("{}", WearableDescriptor::Bloodstained)
        );
        assert_eq!("broken", format!("{}", WearableDescriptor::Broken));
        assert_eq!("colourful", format!("{}", WearableDescriptor::Colourful));
        assert_eq!("dingy", format!("{}", WearableDescriptor::Dingy));
        assert_eq!("drab", format!("{}", WearableDescriptor::Drab));
        assert_eq!("ill fitting", format!("{}", WearableDescriptor::IllFitting));
        assert_eq!(
            "loose fitting",
            format!("{}", WearableDescriptor::LooseFitting)
        );
        assert_eq!("rusty", format!("{}", WearableDescriptor::Rusty));
        assert_eq!("shimmering", format!("{}", WearableDescriptor::Shimmering));
        assert_eq!("shiny", format!("{}", WearableDescriptor::Shiny));
        assert_eq!("stained", format!("{}", WearableDescriptor::Stained));
    }
}

#[cfg(test)]
mod wearable_material_tests {
    use crate::components::wearable::WearableMaterial;

    #[test]
    fn display() {
        assert_eq!("iron", format!("{}", WearableMaterial::Iron));
        assert_eq!("leather", format!("{}", WearableMaterial::Leather));
        assert_eq!("steel", format!("{}", WearableMaterial::Steel));
    }
}

#[cfg(test)]
mod wearable_tests {
    use super::{Wearable, WearableDescriptor, WearableMaterial, WearableType};

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
            material: Some(WearableMaterial::Steel),
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
            descriptors: vec![WearableDescriptor::Dingy, WearableDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("dingy bloodstained shackles", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities_and_material() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(WearableMaterial::Iron),
            descriptors: vec![WearableDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("bloodstained iron shackles", format!("{}", wearable));
    }
}
