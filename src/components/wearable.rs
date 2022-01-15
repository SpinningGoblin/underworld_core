use std::fmt::Display;

use super::defense::Defense;

#[derive(Clone, Debug)]
pub enum WearableType {
    Armour,
    Cloak,
    Clothing,
    PlateMail,
    Shackles,
}

impl Display for WearableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Armour => write!(f, "armour"),
            Self::Cloak => write!(f, "cloak"),
            Self::Clothing => write!(f, "clothing"),
            Self::PlateMail => write!(f, "plate mail"),
            Self::Shackles => write!(f, "shackles"),
        }
    }
}

#[derive(Clone, Debug)]
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
pub struct Wearable {
    pub wearable_type: WearableType,
    pub material: Option<WearableMaterial>,
    pub descriptors: Vec<WearableDescriptor>,
    pub defense: Option<Defense>,
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

#[derive(Clone, Debug)]
pub struct EquippedWearable {
    pub wearable: Wearable,
    pub hidden: bool,
    pub equipped_location: String,
    pub multiple: bool,
}

#[cfg(test)]
mod wearable_type_tests {
    use super::WearableType;

    #[test]
    fn display() {
        assert_eq!("armour", format!("{}", WearableType::Armour));
        assert_eq!("cloak", format!("{}", WearableType::Cloak));
        assert_eq!("clothing", format!("{}", WearableType::Clothing));
        assert_eq!("plate mail", format!("{}", WearableType::PlateMail));
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
            wearable_type: WearableType::PlateMail,
            material: Some(WearableMaterial::Steel),
            descriptors: Vec::new(),
            defense: None,
        };

        assert_eq!("steel plate mail", format!("{}", wearable));
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
