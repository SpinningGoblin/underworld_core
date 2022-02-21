use enum_iterator::IntoEnumIterator;

use crate::components::{
    items::descriptor::Descriptor,
    tag::{Tag, Tagged},
};

pub fn descriptor_for_equipped(descriptor: &Descriptor) -> bool {
    tags_for_descriptor(descriptor).contains(&Tag::Equipped)
}

pub fn matches_tagged(tagged: &impl Tagged) -> Vec<Descriptor> {
    matches_tags(tagged.tags())
}

pub fn matches_two_tagged(tagged_1: &impl Tagged, tagged_2: &impl Tagged) -> Vec<Descriptor> {
    let mut tags = tagged_1.tags();
    let mut tags_2 = tagged_2.tags();
    tags.append(&mut tags_2);
    matches_tags(tags)
}

fn matches_tags(tags: Vec<Tag>) -> Vec<Descriptor> {
    Descriptor::into_enum_iter()
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

#[cfg(test)]
mod tests {
    use crate::{
        components::{items::item_type::ItemType, material::Material},
        generators::utils::item_descriptors::{matches_tagged, matches_two_tagged},
    };

    #[test]
    fn get_descriptors_for_weapon() {
        let descriptors = matches_two_tagged(&ItemType::Whip, &Material::Leather);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_for_wearable() {
        let descriptors = matches_two_tagged(&ItemType::LoinCloth, &Material::Wool);
        assert!(!descriptors.is_empty());
    }

    #[test]
    fn get_descriptors_single_wearable() {
        let descriptors = matches_tagged(&ItemType::LoinCloth);
        assert!(!descriptors.is_empty());
    }
}
