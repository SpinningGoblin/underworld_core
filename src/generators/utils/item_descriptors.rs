use strum::IntoEnumIterator;

use crate::components::{
    items::{Descriptor, ItemType},
    Material, Tag, Tagged,
};

pub fn valid_for_level(descriptor: &Descriptor, level: u32) -> bool {
    match descriptor {
        Descriptor::Beaten
        | Descriptor::Broken
        | Descriptor::Cracked
        | Descriptor::Chipped
        | Descriptor::Crumbling => level <= 5,
        Descriptor::Dingy
        | Descriptor::Dull
        | Descriptor::Moldy
        | Descriptor::Ripped
        | Descriptor::Rotting
        | Descriptor::Rusty
        | Descriptor::Scuffed
        | Descriptor::Soiled
        | Descriptor::Splintered
        | Descriptor::Stained
        | Descriptor::Tangled
        | Descriptor::WaterLogged
        | Descriptor::Torn => level <= 10,
        Descriptor::Heavy | Descriptor::Keen | Descriptor::Shimmering | Descriptor::Shiny => {
            level > 10
        }
        Descriptor::Tarnished => level <= 15,
        Descriptor::Bleached | Descriptor::Drab => level <= 20,
        Descriptor::Quality => level > 20,
        Descriptor::Dirty => level <= 50,
        Descriptor::Bloodstained
        | Descriptor::Colourful
        | Descriptor::Smoothed
        | Descriptor::Weathered => true,
    }
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
        Descriptor::Ripped => vec![Tag::Cloth],
        Descriptor::Rusty => vec![Tag::Metal],
        Descriptor::Scuffed => vec![Tag::Leather],
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
        Descriptor::Soiled => vec![Tag::Cloth, Tag::Clothing],
        Descriptor::Moldy => vec![
            Tag::Cloth,
            Tag::Clothing,
            Tag::Leather,
            Tag::Wood,
            Tag::Paper,
        ],
        Descriptor::Heavy => vec![Tag::Blunt],
        Descriptor::Keen => vec![Tag::Blade],
        Descriptor::Quality => vec![Tag::Blade, Tag::Blunt],
    }
}

pub fn possible_descriptors(
    item_type: &ItemType,
    material: &Option<Material>,
    level: u32,
) -> Vec<Descriptor> {
    match material {
        Some(material) => {
            let tags: Vec<Tag> = item_type
                .tags()
                .into_iter()
                .chain(material.tags().into_iter())
                .collect();
            matches_tags(&tags)
        }
        None => matches_tags(&item_type.tags()),
    }
    .into_iter()
    .filter(|descriptor| valid_for_level(descriptor, level))
    .collect()
}
