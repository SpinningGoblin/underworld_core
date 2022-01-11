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
pub enum WearableQuality {
    Bloodstained,
    Broken,
    Colourful,
    Dingy,
    Drab,
    IllFitting,
    LooseFitting,
    Rusty,
    Shimmering,
    Shiny,
    Stained,
}

impl Display for WearableQuality {
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
    pub qualities: Vec<WearableQuality>,
    pub defense: Option<Defense>,
}

impl Display for Wearable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();

        self.qualities
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
    use super::WearableQuality;

    #[test]
    fn display() {
        assert_eq!("bloodstained", format!("{}", WearableQuality::Bloodstained));
        assert_eq!("broken", format!("{}", WearableQuality::Broken));
        assert_eq!("colourful", format!("{}", WearableQuality::Colourful));
        assert_eq!("dingy", format!("{}", WearableQuality::Dingy));
        assert_eq!("drab", format!("{}", WearableQuality::Drab));
        assert_eq!("ill fitting", format!("{}", WearableQuality::IllFitting));
        assert_eq!(
            "loose fitting",
            format!("{}", WearableQuality::LooseFitting)
        );
        assert_eq!("rusty", format!("{}", WearableQuality::Rusty));
        assert_eq!("shimmering", format!("{}", WearableQuality::Shimmering));
        assert_eq!("shiny", format!("{}", WearableQuality::Shiny));
        assert_eq!("stained", format!("{}", WearableQuality::Stained));
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
    use super::{Wearable, WearableMaterial, WearableQuality, WearableType};

    #[test]
    fn display_when_there_is_only_type() {
        let wearable = Wearable {
            wearable_type: WearableType::Armour,
            material: None,
            qualities: Vec::new(),
            defense: None,
        };

        assert_eq!("armour", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_is_material() {
        let wearable = Wearable {
            wearable_type: WearableType::PlateMail,
            material: Some(WearableMaterial::Steel),
            qualities: Vec::new(),
            defense: None,
        };

        assert_eq!("steel plate mail", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: None,
            qualities: vec![WearableQuality::Dingy, WearableQuality::Bloodstained],
            defense: None,
        };

        assert_eq!("dingy bloodstained shackles", format!("{}", wearable));
    }

    #[test]
    fn display_when_there_are_qualities_and_material() {
        let wearable = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(WearableMaterial::Iron),
            qualities: vec![WearableQuality::Bloodstained],
            defense: None,
        };

        assert_eq!("bloodstained iron shackles", format!("{}", wearable));
    }
}
