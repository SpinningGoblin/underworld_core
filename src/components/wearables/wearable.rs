#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::components::{
    defense::Defense,
    equipment::{location_tag::LocationTag, Equipment},
    material::Material,
    object::Object,
    object_descriptor::ObjectDescriptor,
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
    pub descriptors: Vec<ObjectDescriptor>,
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

impl Equipment for Wearable {
    fn possible_location_tags(&self) -> Vec<LocationTag> {
        self.wearable_type.possible_location_tags()
    }

    fn character_location_tags(&self) -> Vec<LocationTag> {
        self.wearable_type.character_location_tags()
    }
}

impl Object for Wearable {
    fn look_at(&self, is_equipped: bool) -> String {
        self.describe(is_equipped)
    }

    fn material(&self) -> Option<Material> {
        self.material.clone()
    }

    fn is_multiple(&self) -> bool {
        self.wearable_type.is_multiple()
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
    use crate::components::object_descriptor::ObjectDescriptor;

    #[test]
    fn display() {
        assert_eq!(
            "bloodstained",
            format!("{}", ObjectDescriptor::Bloodstained)
        );
        assert_eq!("broken", format!("{}", ObjectDescriptor::Broken));
        assert_eq!("colourful", format!("{}", ObjectDescriptor::Colourful));
        assert_eq!("dingy", format!("{}", ObjectDescriptor::Dingy));
        assert_eq!("drab", format!("{}", ObjectDescriptor::Drab));
        assert_eq!("ill fitting", format!("{}", ObjectDescriptor::IllFitting));
        assert_eq!(
            "loose fitting",
            format!("{}", ObjectDescriptor::LooseFitting)
        );
        assert_eq!("rusty", format!("{}", ObjectDescriptor::Rusty));
        assert_eq!("shimmering", format!("{}", ObjectDescriptor::Shimmering));
        assert_eq!("shiny", format!("{}", ObjectDescriptor::Shiny));
        assert_eq!("stained", format!("{}", ObjectDescriptor::Stained));
    }
}

#[cfg(test)]
mod wearable_tests {
    use crate::components::material::Material;

    use super::{ObjectDescriptor, Wearable, WearableType};

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
            descriptors: vec![ObjectDescriptor::Dingy, ObjectDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("dingy bloodstained shackles", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities_and_material() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(Material::Iron),
            descriptors: vec![ObjectDescriptor::Bloodstained],
            defense: None,
        };

        assert_eq!("bloodstained iron shackles", format!("{}", wearable));
    }
}
