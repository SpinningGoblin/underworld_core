use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::tag::{Tag, Tagged};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Descriptor {
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

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Descriptor::Beaten => write!(f, "beaten"),
            Descriptor::Bleached => write!(f, "bleached"),
            Descriptor::Bloodstained => write!(f, "bloodstained"),
            Descriptor::Broken => write!(f, "broken"),
            Descriptor::Chipped => write!(f, "chipped"),
            Descriptor::Colourful => write!(f, "colourful"),
            Descriptor::Cracked => write!(f, "cracked"),
            Descriptor::Crumbling => write!(f, "crumbling"),
            Descriptor::Dingy => write!(f, "dingy"),
            Descriptor::Dirty => write!(f, "dirty"),
            Descriptor::Drab => write!(f, "drab"),
            Descriptor::Dull => write!(f, "dull"),
            Descriptor::IllFitting => write!(f, "ill fitting"),
            Descriptor::LooseFitting => write!(f, "loose fitting"),
            Descriptor::Ripped => write!(f, "ripped"),
            Descriptor::Rusty => write!(f, "rusty"),
            Descriptor::SetOf => write!(f, "set of"),
            Descriptor::Shimmering => write!(f, "shimmering"),
            Descriptor::Shiny => write!(f, "shiny"),
            Descriptor::Scuffed => write!(f, "scuffed"),
            Descriptor::Smoothed => write!(f, "smoothed"),
            Descriptor::Splintered => write!(f, "splintered"),
            Descriptor::Stained => write!(f, "stained"),
            Descriptor::Tangled => write!(f, "tangled"),
            Descriptor::Tarnished => write!(f, "tarnished"),
            Descriptor::Torn => write!(f, "torn"),
            Descriptor::WaterLogged => write!(f, "water logged"),
            Descriptor::Weathered => write!(f, "weathered"),
            Descriptor::Rotting => write!(f, "rotting"),
        }
    }
}

impl Descriptor {
    pub fn is_for_equipped(&self) -> bool {
        self.descriptor_tags().contains(&Tag::Equipped)
    }

    fn descriptor_tags(&self) -> Vec<Tag> {
        match *self {
            Descriptor::Beaten => vec![Tag::Wood, Tag::Bone, Tag::Leather],
            Descriptor::Bleached => vec![Tag::Bone, Tag::Wood],
            Descriptor::Bloodstained => vec![
                Tag::Blade,
                Tag::Blunt,
                Tag::Armour,
                Tag::Clothing,
                Tag::Fixture,
            ],
            Descriptor::Broken => vec![
                Tag::Armour,
                Tag::Blade,
                Tag::Blunt,
                Tag::Shield,
                Tag::Container,
            ],
            Descriptor::Chipped => vec![Tag::Blade, Tag::Blunt, Tag::Bone],
            Descriptor::Colourful => vec![Tag::Cloth, Tag::Clothing],
            Descriptor::Cracked => vec![Tag::Bone, Tag::Stone],
            Descriptor::Crumbling => vec![Tag::Leather],
            Descriptor::Dingy => {
                vec![Tag::Cloth, Tag::Clothing, Tag::Leather]
            }
            Descriptor::Dirty => {
                vec![Tag::Cloth, Tag::Clothing, Tag::Fixture]
            }
            Descriptor::Drab => vec![Tag::Clothing],
            Descriptor::Dull => vec![Tag::Blade],
            Descriptor::IllFitting => {
                vec![Tag::Armour, Tag::Clothing, Tag::Equipped]
            }
            Descriptor::LooseFitting => {
                vec![Tag::Armour, Tag::Clothing, Tag::Equipped]
            }
            Descriptor::Ripped => vec![Tag::Cloth],
            Descriptor::Rusty => vec![Tag::Metal],
            Descriptor::Scuffed => vec![Tag::Leather],
            Descriptor::SetOf => vec![],
            Descriptor::Shimmering => vec![Tag::Cloth, Tag::Clothing],
            Descriptor::Shiny => vec![Tag::Metal],
            Descriptor::Smoothed => vec![Tag::Bone, Tag::Stone, Tag::Wood],
            Descriptor::Splintered => vec![Tag::Wood],
            Descriptor::Stained => {
                vec![Tag::Cloth, Tag::Clothing, Tag::Leather]
            }
            Descriptor::Tangled => vec![Tag::Rope],
            Descriptor::Tarnished => vec![Tag::Metal],
            Descriptor::Torn => vec![Tag::Cloth, Tag::Clothing],
            Descriptor::WaterLogged => {
                vec![Tag::Cloth, Tag::Clothing, Tag::Leather]
            }
            Descriptor::Weathered => {
                vec![Tag::Bone, Tag::Leather, Tag::Wood]
            }
            Descriptor::Rotting => vec![Tag::Cloth, Tag::Clothing],
        }
    }

    pub fn matches_tagged(tagged: &impl Tagged) -> Vec<Descriptor> {
        Self::matches_tags(tagged.tags())
    }

    pub fn matches_two_tagged(tagged_1: &impl Tagged, tagged_2: &impl Tagged) -> Vec<Descriptor> {
        let mut tags = tagged_1.tags();
        let mut tags_2 = tagged_2.tags();
        tags.append(&mut tags_2);
        Self::matches_tags(tags)
    }

    fn matches_tags(tags: Vec<Tag>) -> Vec<Descriptor> {
        Descriptor::into_enum_iter()
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
    use crate::components::{items::item_type::ItemType, material::Material};

    use super::Descriptor;

    #[test]
    fn get_descriptors_for_weapon() {
        let descriptors = Descriptor::matches_two_tagged(&ItemType::Whip, &Material::Leather);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_for_wearable() {
        let descriptors = Descriptor::matches_two_tagged(&ItemType::LoinCloth, &Material::Wool);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_single_wearable() {
        let descriptors = Descriptor::matches_tagged(&ItemType::LoinCloth);
        assert!(!descriptors.is_empty());
    }
}
