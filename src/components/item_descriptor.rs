use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_tags::{DescriptorTag, DescriptorTagged};

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
        }
    }
}

impl ItemDescriptor {
    fn descriptor_tags(&self) -> Vec<DescriptorTag> {
        match *self {
            ItemDescriptor::Beaten => vec![
                DescriptorTag::Wood,
                DescriptorTag::Bone,
                DescriptorTag::Leather,
            ],
            ItemDescriptor::Bleached => vec![DescriptorTag::Bone, DescriptorTag::Wood],
            ItemDescriptor::Bloodstained => vec![
                DescriptorTag::Blade,
                DescriptorTag::Blunt,
                DescriptorTag::Armour,
                DescriptorTag::Clothing,
            ],
            ItemDescriptor::Broken => vec![
                DescriptorTag::Armour,
                DescriptorTag::Blade,
                DescriptorTag::Blunt,
                DescriptorTag::Shield,
            ],
            ItemDescriptor::Chipped => vec![
                DescriptorTag::Blade,
                DescriptorTag::Blunt,
                DescriptorTag::Bone,
            ],
            ItemDescriptor::Colourful => vec![DescriptorTag::Cloth, DescriptorTag::Clothing],
            ItemDescriptor::Cracked => vec![DescriptorTag::Bone, DescriptorTag::Stone],
            ItemDescriptor::Crumbling => vec![DescriptorTag::Leather],
            ItemDescriptor::Dingy => vec![
                DescriptorTag::Cloth,
                DescriptorTag::Clothing,
                DescriptorTag::Leather,
            ],
            ItemDescriptor::Dirty => vec![DescriptorTag::Cloth, DescriptorTag::Clothing],
            ItemDescriptor::Drab => vec![DescriptorTag::Clothing],
            ItemDescriptor::Dull => vec![DescriptorTag::Blade],
            ItemDescriptor::IllFitting => vec![DescriptorTag::Armour, DescriptorTag::Clothing],
            ItemDescriptor::LooseFitting => vec![DescriptorTag::Armour, DescriptorTag::Clothing],
            ItemDescriptor::Ripped => vec![DescriptorTag::Cloth],
            ItemDescriptor::Rusty => vec![DescriptorTag::Metal],
            ItemDescriptor::Scuffed => vec![DescriptorTag::Leather],
            ItemDescriptor::SetOf => vec![],
            ItemDescriptor::Shimmering => vec![DescriptorTag::Cloth, DescriptorTag::Clothing],
            ItemDescriptor::Shiny => vec![DescriptorTag::Metal],
            ItemDescriptor::Smoothed => vec![
                DescriptorTag::Bone,
                DescriptorTag::Stone,
                DescriptorTag::Wood,
            ],
            ItemDescriptor::Splintered => vec![DescriptorTag::Wood],
            ItemDescriptor::Stained => vec![
                DescriptorTag::Cloth,
                DescriptorTag::Clothing,
                DescriptorTag::Leather,
            ],
            ItemDescriptor::Tangled => vec![DescriptorTag::Rope],
            ItemDescriptor::Tarnished => vec![DescriptorTag::Metal],
            ItemDescriptor::Torn => vec![DescriptorTag::Cloth, DescriptorTag::Clothing],
            ItemDescriptor::WaterLogged => vec![
                DescriptorTag::Cloth,
                DescriptorTag::Clothing,
                DescriptorTag::Leather,
            ],
            ItemDescriptor::Weathered => vec![
                DescriptorTag::Bone,
                DescriptorTag::Leather,
                DescriptorTag::Wood,
            ],
        }
    }

    fn has_tag(&self, tag: &DescriptorTag) -> bool {
        self.descriptor_tags().contains(tag)
    }

    pub fn matches_tagged(tagged: &impl DescriptorTagged) -> Vec<ItemDescriptor> {
        Self::matches_tag(&tagged.descriptor_tag())
    }

    pub fn matches_two_tagged(
        tagged_1: &impl DescriptorTagged,
        tagged_2: &impl DescriptorTagged,
    ) -> Vec<ItemDescriptor> {
        Self::matches_tags(&tagged_1.descriptor_tag(), &tagged_2.descriptor_tag())
    }

    fn matches_tags(tag_1: &DescriptorTag, tag_2: &DescriptorTag) -> Vec<ItemDescriptor> {
        ItemDescriptor::into_enum_iter()
            .filter(|descriptor| descriptor.has_tag(tag_1) || descriptor.has_tag(tag_2))
            .collect()
    }

    fn matches_tag(tag: &DescriptorTag) -> Vec<ItemDescriptor> {
        ItemDescriptor::into_enum_iter()
            .filter(|descriptor| descriptor.has_tag(tag))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        item_material::ItemMaterial, weapons::weapon_type::WeaponType,
        wearables::wearable_type::WearableType,
    };

    use super::ItemDescriptor;

    #[test]
    fn get_descriptors_for_weapon() {
        let descriptors =
            ItemDescriptor::matches_two_tagged(&WeaponType::Whip, &ItemMaterial::Leather);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_for_wearable() {
        let descriptors =
            ItemDescriptor::matches_two_tagged(&WearableType::LoinCloth, &ItemMaterial::Wool);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_single_wearable() {
        let descriptors = ItemDescriptor::matches_tagged(&WearableType::LoinCloth);
        assert!(!descriptors.is_empty());
    }
}
