use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::item_tag::{ItemTag, TaggedItem};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum ItemDescriptor {
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

impl Display for ItemDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ItemDescriptor::Beaten => write!(f, "beaten"),
            ItemDescriptor::Bleached => write!(f, "bleached"),
            ItemDescriptor::Bloodstained => write!(f, "bloodstained"),
            ItemDescriptor::Broken => write!(f, "broken"),
            ItemDescriptor::Chipped => write!(f, "chipped"),
            ItemDescriptor::Colourful => write!(f, "colourful"),
            ItemDescriptor::Cracked => write!(f, "cracked"),
            ItemDescriptor::Crumbling => write!(f, "crumbling"),
            ItemDescriptor::Dingy => write!(f, "dingy"),
            ItemDescriptor::Dirty => write!(f, "dirty"),
            ItemDescriptor::Drab => write!(f, "drab"),
            ItemDescriptor::Dull => write!(f, "dull"),
            ItemDescriptor::IllFitting => write!(f, "ill fitting"),
            ItemDescriptor::LooseFitting => write!(f, "loose fitting"),
            ItemDescriptor::Ripped => write!(f, "ripped"),
            ItemDescriptor::Rusty => write!(f, "rusty"),
            ItemDescriptor::SetOf => write!(f, "set of"),
            ItemDescriptor::Shimmering => write!(f, "shimmering"),
            ItemDescriptor::Shiny => write!(f, "shiny"),
            ItemDescriptor::Scuffed => write!(f, "scuffed"),
            ItemDescriptor::Smoothed => write!(f, "smoothed"),
            ItemDescriptor::Splintered => write!(f, "splintered"),
            ItemDescriptor::Stained => write!(f, "stained"),
            ItemDescriptor::Tangled => write!(f, "tangled"),
            ItemDescriptor::Tarnished => write!(f, "tarnished"),
            ItemDescriptor::Torn => write!(f, "torn"),
            ItemDescriptor::WaterLogged => write!(f, "water logged"),
            ItemDescriptor::Weathered => write!(f, "weathered"),
            ItemDescriptor::Rotting => write!(f, "rotting"),
        }
    }
}

impl ItemDescriptor {
    pub fn is_for_equipped(&self) -> bool {
        self.descriptor_tags().contains(&ItemTag::Equipped)
    }

    fn descriptor_tags(&self) -> Vec<ItemTag> {
        match *self {
            ItemDescriptor::Beaten => vec![ItemTag::Wood, ItemTag::Bone, ItemTag::Leather],
            ItemDescriptor::Bleached => vec![ItemTag::Bone, ItemTag::Wood],
            ItemDescriptor::Bloodstained => vec![
                ItemTag::Blade,
                ItemTag::Blunt,
                ItemTag::Armour,
                ItemTag::Clothing,
            ],
            ItemDescriptor::Broken => vec![
                ItemTag::Armour,
                ItemTag::Blade,
                ItemTag::Blunt,
                ItemTag::Shield,
            ],
            ItemDescriptor::Chipped => vec![ItemTag::Blade, ItemTag::Blunt, ItemTag::Bone],
            ItemDescriptor::Colourful => vec![ItemTag::Cloth, ItemTag::Clothing],
            ItemDescriptor::Cracked => vec![ItemTag::Bone, ItemTag::Stone],
            ItemDescriptor::Crumbling => vec![ItemTag::Leather],
            ItemDescriptor::Dingy => vec![ItemTag::Cloth, ItemTag::Clothing, ItemTag::Leather],
            ItemDescriptor::Dirty => vec![ItemTag::Cloth, ItemTag::Clothing],
            ItemDescriptor::Drab => vec![ItemTag::Clothing],
            ItemDescriptor::Dull => vec![ItemTag::Blade],
            ItemDescriptor::IllFitting => {
                vec![ItemTag::Armour, ItemTag::Clothing, ItemTag::Equipped]
            }
            ItemDescriptor::LooseFitting => {
                vec![ItemTag::Armour, ItemTag::Clothing, ItemTag::Equipped]
            }
            ItemDescriptor::Ripped => vec![ItemTag::Cloth],
            ItemDescriptor::Rusty => vec![ItemTag::Metal],
            ItemDescriptor::Scuffed => vec![ItemTag::Leather],
            ItemDescriptor::SetOf => vec![],
            ItemDescriptor::Shimmering => vec![ItemTag::Cloth, ItemTag::Clothing],
            ItemDescriptor::Shiny => vec![ItemTag::Metal],
            ItemDescriptor::Smoothed => vec![ItemTag::Bone, ItemTag::Stone, ItemTag::Wood],
            ItemDescriptor::Splintered => vec![ItemTag::Wood],
            ItemDescriptor::Stained => vec![ItemTag::Cloth, ItemTag::Clothing, ItemTag::Leather],
            ItemDescriptor::Tangled => vec![ItemTag::Rope],
            ItemDescriptor::Tarnished => vec![ItemTag::Metal],
            ItemDescriptor::Torn => vec![ItemTag::Cloth, ItemTag::Clothing],
            ItemDescriptor::WaterLogged => {
                vec![ItemTag::Cloth, ItemTag::Clothing, ItemTag::Leather]
            }
            ItemDescriptor::Weathered => vec![ItemTag::Bone, ItemTag::Leather, ItemTag::Wood],
            ItemDescriptor::Rotting => vec![ItemTag::Cloth, ItemTag::Clothing],
        }
    }

    pub fn matches_tagged(tagged: &impl TaggedItem) -> Vec<ItemDescriptor> {
        Self::matches_tags(tagged.tags())
    }

    pub fn matches_two_tagged(
        tagged_1: &impl TaggedItem,
        tagged_2: &impl TaggedItem,
    ) -> Vec<ItemDescriptor> {
        let mut tags = tagged_1.tags();
        let mut tags_2 = tagged_2.tags();
        tags.append(&mut tags_2);
        Self::matches_tags(tags)
    }

    fn matches_tags(tags: Vec<ItemTag>) -> Vec<ItemDescriptor> {
        ItemDescriptor::into_enum_iter()
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

    use super::ItemDescriptor;

    #[test]
    fn get_descriptors_for_weapon() {
        let descriptors = ItemDescriptor::matches_two_tagged(&WeaponType::Whip, &Material::Leather);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_for_wearable() {
        let descriptors =
            ItemDescriptor::matches_two_tagged(&WearableType::LoinCloth, &Material::Wool);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_single_wearable() {
        let descriptors = ItemDescriptor::matches_tagged(&WearableType::LoinCloth);
        assert!(!descriptors.is_empty());
    }
}
