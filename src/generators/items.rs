use rand::Rng;
use std::ops::RangeInclusive;

use crate::components::{
    damage::{Attack, Defense},
    identifier::Identifier,
    items::{descriptor::Descriptor, item::Item, item_type::ItemType},
    material::Material,
    tag::{Tag, Tagged},
};

use super::{
    generator::Generator,
    utils::item_descriptors::{descriptor_for_equipped, matches_tags},
};

pub fn item_generator(item_type: &ItemType, is_equipped: bool) -> impl Generator<Item> {
    ItemPrototype {
        item_type: item_type.clone(),
        num_descriptors: 1..=2,
        materials: super::utils::materials::possible_materials(item_type),
        is_equipped,
    }
}

pub struct ItemPrototype {
    pub item_type: ItemType,
    pub num_descriptors: RangeInclusive<usize>,
    pub materials: Vec<Material>,
    pub is_equipped: bool,
}

impl Generator<Item> for ItemPrototype {
    fn generate(&self) -> Item {
        let material = self.material();
        let descriptors = self.descriptors(&material);
        let attack = self.num_attack_rolls().map(|num_rolls| Attack {
            num_rolls,
            modifier: self.attack_modifier().unwrap_or_default(),
        });
        let defense = self.resistance().map(|resistance| Defense {
            damage_resistance: resistance,
        });
        let tags = self.item_type.tags();

        Item {
            identifier: Identifier::just_id(),
            item_type: self.item_type.clone(),
            tags,
            descriptors,
            material,
            attack,
            defense,
            consumable_effect: None,
        }
    }
}

impl ItemPrototype {
    fn material(&self) -> Option<Material> {
        if self.materials.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.materials.len());
            self.materials.get(index).cloned()
        }
    }

    fn descriptors(&self, material: &Option<Material>) -> Vec<Descriptor> {
        let mut rng = rand::thread_rng();
        let num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let num_descriptor_range = 0..num_descriptors;

        if num_descriptor_range.is_empty() {
            return Vec::new();
        }

        let mut possible_descriptors: Vec<Descriptor> = self.possible_descriptors(material);
        let descriptors = num_descriptor_range.filter_map(|_| {
            if possible_descriptors.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_descriptors.len());
                Some(possible_descriptors.remove(index))
            }
        });

        descriptors
            .into_iter()
            .chain(self.necessary_descriptors().into_iter())
            .collect()
    }

    fn necessary_descriptors(&self) -> Vec<Descriptor> {
        match self.item_type {
            ItemType::Shackles => vec![Descriptor::SetOf],
            _ => Vec::new(),
        }
    }

    fn possible_descriptors(&self, material: &Option<Material>) -> Vec<Descriptor> {
        match material {
            Some(material) => {
                let tags: Vec<Tag> = self
                    .item_type
                    .tags()
                    .into_iter()
                    .chain(material.tags().into_iter())
                    .collect();
                matches_tags(&tags)
            }
            None => matches_tags(&self.item_type.tags()),
        }
        .into_iter()
        .filter(|descriptor| self.is_equipped || !descriptor_for_equipped(descriptor))
        .collect()
    }

    fn num_attack_rolls(&self) -> Option<usize> {
        match self.item_type {
            ItemType::Buckler
            | ItemType::Dagger
            | ItemType::Dirk
            | ItemType::Shield
            | ItemType::ShortSword => Some(1),
            ItemType::Club
            | ItemType::Hammer
            | ItemType::LongSword
            | ItemType::Mace
            | ItemType::Morningstar
            | ItemType::Whip => Some(2),
            ItemType::GreatSword => Some(3),
            _ => None,
        }
    }

    fn attack_modifier(&self) -> Option<i32> {
        match self.item_type {
            ItemType::Buckler
            | ItemType::Shield
            | ItemType::ShortSword
            | ItemType::Dagger
            | ItemType::Dirk => Some(-1),
            ItemType::GreatSword => Some(2),
            _ => None,
        }
    }

    fn resistance(&self) -> Option<i32> {
        match self.item_type {
            ItemType::Boots | ItemType::Buckler | ItemType::Shield | ItemType::Vest => Some(2),
            ItemType::Shirt
            | ItemType::Gloves
            | ItemType::Trousers
            | ItemType::Cloak
            | ItemType::LoinCloth
            | ItemType::Shackles => Some(1),
            _ => None,
        }
    }
}
