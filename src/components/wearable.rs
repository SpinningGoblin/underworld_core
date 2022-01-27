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
    Armour,
    Cloak,
    Clothing,
    PlateMailHelmet,
    Shackles,
}

impl WearableType {
    pub fn necessary_descriptors(&self) -> Vec<WearableDescriptor> {
        match *self {
            WearableType::Armour => Vec::new(),
            WearableType::Cloak => Vec::new(),
            WearableType::Clothing => Vec::new(),
            WearableType::PlateMailHelmet => Vec::new(),
            WearableType::Shackles => vec![WearableDescriptor::SetOf],
        }
    }

    pub fn possible_descriptors(&self) -> Vec<WearableDescriptor> {
        match *self {
            WearableType::Armour => vec![
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
            WearableType::Clothing => vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
            WearableType::PlateMailHelmet => vec![
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
        }
    }
}

impl Equippable for WearableType {
    fn possible_equip_locations(&self) -> Vec<EquippedLocation> {
        match *self {
            WearableType::Armour => Vec::new(),
            WearableType::Cloak => vec![EquippedLocation::HangingLooselyShoulders],
            WearableType::Clothing => Vec::new(),
            WearableType::PlateMailHelmet => Vec::new(),
            WearableType::Shackles => Vec::new(),
        }
    }

    fn is_multiple(&self) -> bool {
        match *self {
            WearableType::Armour => true,
            WearableType::Cloak => false,
            WearableType::Clothing => true,
            WearableType::PlateMailHelmet => false,
            WearableType::Shackles => false,
        }
    }
}

impl Display for WearableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Armour => write!(f, "armour"),
            Self::Cloak => write!(f, "cloak"),
            Self::Clothing => write!(f, "clothing"),
            Self::PlateMailHelmet => write!(f, "plate mail helmet"),
            Self::Shackles => write!(f, "shackles"),
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
    Iron,
    Leather,
    Steel,
}

impl Display for WearableMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Iron => write!(f, "iron"),
            Self::Leather => write!(f, "leather"),
            Self::Steel => write!(f, "steel"),
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
        assert_eq!("armour", format!("{}", WearableType::Armour));
        assert_eq!("cloak", format!("{}", WearableType::Cloak));
        assert_eq!("clothing", format!("{}", WearableType::Clothing));
        assert_eq!(
            "plate mail helmet",
            format!("{}", WearableType::PlateMailHelmet)
        );
        assert_eq!("shackles", format!("{}", WearableType::Shackles));
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
mod weapon_material_tests {
    use crate::components::wearable::WearableMaterial;

    #[test]
    fn display() {
        assert_eq!("iron", format!("{}", WearableMaterial::Iron));
        assert_eq!("leather", format!("{}", WearableMaterial::Leather));
        assert_eq!("steel", format!("{}", WearableMaterial::Steel));
    }
}

#[cfg(test)]
mod weapon_tests {
    use super::{Wearable, WearableDescriptor, WearableMaterial, WearableType};

    #[test]
    fn display_when_there_is_only_type() {
        let wearable = Wearable {
            wearable_type: WearableType::Armour,
            material: None,
            descriptors: Vec::new(),
            defense: None,
        };

        assert_eq!("armour", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_is_material() {
        let wearable = Wearable {
            wearable_type: WearableType::PlateMailHelmet,
            material: Some(WearableMaterial::Steel),
            descriptors: Vec::new(),
            defense: None,
        };

        assert_eq!("steel plate mail helmet", format!("{}", wearable));
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
