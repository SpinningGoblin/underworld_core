#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    defense::Defense,
    equipped::equip_location_descriptor::EquipLocationDescriptor,
    item::{EquippableItem, Item},
    item_descriptor::ItemDescriptor,
    material::Material,
};

use super::wearable_type::WearableType;

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
    pub material: Option<Material>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<ItemDescriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub defense: Option<Defense>,
}

impl Wearable {
    fn describe(&self, is_equipped: bool) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        self.descriptors
            .iter()
            .filter(|descriptor| {
                if !is_equipped {
                    !descriptor.is_for_equipped()
                } else {
                    true
                }
            })
            .for_each(|descriptor| descriptions.push(descriptor.to_string()));

        if let Some(material) = &self.material {
            descriptions.push(material.to_string());
        }

        descriptions.push(self.wearable_type.to_string());
        descriptions.join(" ")
    }
}

impl EquippableItem for Wearable {
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor> {
        self.wearable_type.possible_equip_locations()
    }
}

impl Item for Wearable {
    fn look_at(&self, is_equipped: bool) -> String {
        self.describe(is_equipped)
    }

    fn is_multiple(&self) -> bool {
        self.wearable_type.is_multiple()
    }

    fn material(&self) -> Option<Material> {
        self.material.clone()
    }
}

impl Display for Wearable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe(false))
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
    use crate::components::material::Material;

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
            material: Some(Material::Steel),
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
            material: Some(Material::Iron),
            descriptors: vec![ItemDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("bloodstained iron shackles", format!("{}", wearable));
    }
}
