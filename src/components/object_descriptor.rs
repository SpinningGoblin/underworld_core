use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::object_tag::{ObjectTag, TaggedObject};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum ObjectDescriptor {
    Beaten,
    Bleached,
    Bloodstained,
    Broken,
    Chipped,
    Colourful,
    Cracked,
    Crumbling,
    Dingy,
    Dirty,
    Drab,
    Dull,
    IllFitting,
    LooseFitting,
    Ripped,
    Rotting,
    Rusty,
    Scuffed,
    SetOf,
    Shimmering,
    Shiny,
    Smoothed,
    Splintered,
    Stained,
    Tangled,
    Tarnished,
    Torn,
    WaterLogged,
    Weathered,
}

impl Display for ObjectDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ObjectDescriptor::Beaten => write!(f, "beaten"),
            ObjectDescriptor::Bleached => write!(f, "bleached"),
            ObjectDescriptor::Bloodstained => write!(f, "bloodstained"),
            ObjectDescriptor::Broken => write!(f, "broken"),
            ObjectDescriptor::Chipped => write!(f, "chipped"),
            ObjectDescriptor::Colourful => write!(f, "colourful"),
            ObjectDescriptor::Cracked => write!(f, "cracked"),
            ObjectDescriptor::Crumbling => write!(f, "crumbling"),
            ObjectDescriptor::Dingy => write!(f, "dingy"),
            ObjectDescriptor::Dirty => write!(f, "dirty"),
            ObjectDescriptor::Drab => write!(f, "drab"),
            ObjectDescriptor::Dull => write!(f, "dull"),
            ObjectDescriptor::IllFitting => write!(f, "ill fitting"),
            ObjectDescriptor::LooseFitting => write!(f, "loose fitting"),
            ObjectDescriptor::Ripped => write!(f, "ripped"),
            ObjectDescriptor::Rusty => write!(f, "rusty"),
            ObjectDescriptor::SetOf => write!(f, "set of"),
            ObjectDescriptor::Shimmering => write!(f, "shimmering"),
            ObjectDescriptor::Shiny => write!(f, "shiny"),
            ObjectDescriptor::Scuffed => write!(f, "scuffed"),
            ObjectDescriptor::Smoothed => write!(f, "smoothed"),
            ObjectDescriptor::Splintered => write!(f, "splintered"),
            ObjectDescriptor::Stained => write!(f, "stained"),
            ObjectDescriptor::Tangled => write!(f, "tangled"),
            ObjectDescriptor::Tarnished => write!(f, "tarnished"),
            ObjectDescriptor::Torn => write!(f, "torn"),
            ObjectDescriptor::WaterLogged => write!(f, "water logged"),
            ObjectDescriptor::Weathered => write!(f, "weathered"),
            ObjectDescriptor::Rotting => write!(f, "rotting"),
        }
    }
}

impl ObjectDescriptor {
    pub fn is_for_equipped(&self) -> bool {
        self.descriptor_tags().contains(&ObjectTag::Equipped)
    }

    fn descriptor_tags(&self) -> Vec<ObjectTag> {
        match *self {
            ObjectDescriptor::Beaten => vec![ObjectTag::Wood, ObjectTag::Bone, ObjectTag::Leather],
            ObjectDescriptor::Bleached => vec![ObjectTag::Bone, ObjectTag::Wood],
            ObjectDescriptor::Bloodstained => vec![
                ObjectTag::Blade,
                ObjectTag::Blunt,
                ObjectTag::Armour,
                ObjectTag::Clothing,
            ],
            ObjectDescriptor::Broken => vec![
                ObjectTag::Armour,
                ObjectTag::Blade,
                ObjectTag::Blunt,
                ObjectTag::Shield,
            ],
            ObjectDescriptor::Chipped => vec![ObjectTag::Blade, ObjectTag::Blunt, ObjectTag::Bone],
            ObjectDescriptor::Colourful => vec![ObjectTag::Cloth, ObjectTag::Clothing],
            ObjectDescriptor::Cracked => vec![ObjectTag::Bone, ObjectTag::Stone],
            ObjectDescriptor::Crumbling => vec![ObjectTag::Leather],
            ObjectDescriptor::Dingy => {
                vec![ObjectTag::Cloth, ObjectTag::Clothing, ObjectTag::Leather]
            }
            ObjectDescriptor::Dirty => vec![ObjectTag::Cloth, ObjectTag::Clothing],
            ObjectDescriptor::Drab => vec![ObjectTag::Clothing],
            ObjectDescriptor::Dull => vec![ObjectTag::Blade],
            ObjectDescriptor::IllFitting => {
                vec![ObjectTag::Armour, ObjectTag::Clothing, ObjectTag::Equipped]
            }
            ObjectDescriptor::LooseFitting => {
                vec![ObjectTag::Armour, ObjectTag::Clothing, ObjectTag::Equipped]
            }
            ObjectDescriptor::Ripped => vec![ObjectTag::Cloth],
            ObjectDescriptor::Rusty => vec![ObjectTag::Metal],
            ObjectDescriptor::Scuffed => vec![ObjectTag::Leather],
            ObjectDescriptor::SetOf => vec![],
            ObjectDescriptor::Shimmering => vec![ObjectTag::Cloth, ObjectTag::Clothing],
            ObjectDescriptor::Shiny => vec![ObjectTag::Metal],
            ObjectDescriptor::Smoothed => vec![ObjectTag::Bone, ObjectTag::Stone, ObjectTag::Wood],
            ObjectDescriptor::Splintered => vec![ObjectTag::Wood],
            ObjectDescriptor::Stained => {
                vec![ObjectTag::Cloth, ObjectTag::Clothing, ObjectTag::Leather]
            }
            ObjectDescriptor::Tangled => vec![ObjectTag::Rope],
            ObjectDescriptor::Tarnished => vec![ObjectTag::Metal],
            ObjectDescriptor::Torn => vec![ObjectTag::Cloth, ObjectTag::Clothing],
            ObjectDescriptor::WaterLogged => {
                vec![ObjectTag::Cloth, ObjectTag::Clothing, ObjectTag::Leather]
            }
            ObjectDescriptor::Weathered => {
                vec![ObjectTag::Bone, ObjectTag::Leather, ObjectTag::Wood]
            }
            ObjectDescriptor::Rotting => vec![ObjectTag::Cloth, ObjectTag::Clothing],
        }
    }

    pub fn matches_tagged(tagged: &impl TaggedObject) -> Vec<ObjectDescriptor> {
        Self::matches_tags(tagged.tags())
    }

    pub fn matches_two_tagged(
        tagged_1: &impl TaggedObject,
        tagged_2: &impl TaggedObject,
    ) -> Vec<ObjectDescriptor> {
        let mut tags = tagged_1.tags();
        let mut tags_2 = tagged_2.tags();
        tags.append(&mut tags_2);
        Self::matches_tags(tags)
    }

    fn matches_tags(tags: Vec<ObjectTag>) -> Vec<ObjectDescriptor> {
        ObjectDescriptor::into_enum_iter()
            .filter(|descriptor| {
                descriptor
                    .descriptor_tags()
                    .iter()
                    .any(|tag| tags.contains(tag))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        material::Material, weapons::weapon_type::WeaponType,
        wearables::wearable_type::WearableType,
    };

    use super::ObjectDescriptor;

    #[test]
    fn get_descriptors_for_weapon() {
        let descriptors =
            ObjectDescriptor::matches_two_tagged(&WeaponType::Whip, &Material::Leather);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_for_wearable() {
        let descriptors =
            ObjectDescriptor::matches_two_tagged(&WearableType::LoinCloth, &Material::Wool);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_single_wearable() {
        let descriptors = ObjectDescriptor::matches_tagged(&WearableType::LoinCloth);
        assert!(!descriptors.is_empty());
    }
}
