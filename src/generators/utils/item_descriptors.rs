use strum::IntoEnumIterator;

use crate::components::{items::descriptor::Descriptor, tag::Tag};

pub fn descriptor_for_equipped(descriptor: &Descriptor) -> bool {
    tags_for_descriptor(descriptor).contains(&Tag::Equipped)
}

pub fn matches_tags(tags: &[Tag]) -> Vec<Descriptor> {
   Descriptor::iter()
        .filter(|descriptor| {
            tags_for_descriptor(descriptor)
                .iter()
                .any(|tag| tags.contains(tag))
        })
        .collect()
}

fn tags_for_descriptor(descriptor: &Descriptor) -> Vec<Tag> {
    match *descriptor {
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
