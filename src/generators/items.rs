use rand::Rng;
use std::ops::RangeInclusive;
use uuid::Uuid;

use crate::components::{
    damage::{Attack, Defense},
    items::{descriptor::Descriptor, item::Item, item_type::ItemType},
    material::Material,
    tag::{Tag, Tagged},
};

use super::{
    generator::Generator,
    utils::item_descriptors::{matches_tags, valid_for_level},
};

pub fn item_generator(item_type: &ItemType, is_equipped: bool) -> impl Generator<Item> {
    ItemPrototype {
        item_type: item_type.clone(),
        num_descriptors: 1..=2,
        materials: super::utils::materials::possible_materials(item_type),
        is_equipped,
        item_level: None,
    }
}

pub fn item_generator_for_level(
    item_type: &ItemType,
    is_equipped: bool,
    level: u32,
) -> impl Generator<Item> {
    ItemPrototype {
        item_type: item_type.clone(),
        num_descriptors: 1..=2,
        materials: super::utils::materials::possible_materials(item_type),
        is_equipped,
        item_level: Some(level),
    }
}

pub struct ItemPrototype {
    pub item_type: ItemType,
    pub num_descriptors: RangeInclusive<usize>,
    pub materials: Vec<Material>,
    pub is_equipped: bool,
    pub item_level: Option<u32>,
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
            id: Uuid::new_v4(),
            name: None,
            item_type: self.item_type.clone(),
            tags,
            descriptors,
            material,
            attack,
            defense,
            consumable: None,
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
        .filter(|descriptor| valid_for_level(descriptor, self.level()))
        .collect()
    }

    fn num_attack_rolls(&self) -> Option<usize> {
        let base_rolls = match self.item_type {
            ItemType::Buckler
            | ItemType::Dagger
            | ItemType::Dirk
            | ItemType::Shield
            | ItemType::ShortSword => 2,
            ItemType::Club
            | ItemType::Hammer
            | ItemType::LongSword
            | ItemType::Mace
            | ItemType::Morningstar
            | ItemType::Whip
            | ItemType::Spear
            | ItemType::Pike => 3,
            ItemType::GreatSword | ItemType::Halberd => 4,
            ItemType::Breastplate
            | ItemType::Boots
            | ItemType::BowlerHat
            | ItemType::Cloak
            | ItemType::Crown
            | ItemType::Fedora
            | ItemType::Gloves
            | ItemType::Helm
            | ItemType::LoinCloth
            | ItemType::Mask
            | ItemType::PlateBoots
            | ItemType::PlateGauntlets
            | ItemType::PlateHelmet
            | ItemType::Scroll
            | ItemType::Shirt
            | ItemType::Shackles
            | ItemType::TopHat
            | ItemType::Trousers
            | ItemType::Vest => return None,
        };

        let level = self.level();
        let roll_modifier = if level == 1 {
            0
        } else if level >= 2 && level <= 10 {
            1
        } else if level >= 11 && level <= 20 {
            3
        } else {
            5
        };

        Some(base_rolls + roll_modifier)
    }

    fn attack_modifier(&self) -> Option<i32> {
        let base_modifier = match self.item_type {
            ItemType::Buckler
            | ItemType::Shield
            | ItemType::ShortSword
            | ItemType::Dagger
            | ItemType::Dirk => -1,
            ItemType::LongSword
            | ItemType::Spear
            | ItemType::Pike
            | ItemType::Hammer
            | ItemType::Morningstar
            | ItemType::Whip => 1,
            ItemType::GreatSword | ItemType::Halberd => 2,
            ItemType::Breastplate
            | ItemType::Boots
            | ItemType::BowlerHat
            | ItemType::Cloak
            | ItemType::Club
            | ItemType::Crown
            | ItemType::Fedora
            | ItemType::Gloves
            | ItemType::Helm
            | ItemType::LoinCloth
            | ItemType::Mace
            | ItemType::Mask
            | ItemType::PlateBoots
            | ItemType::PlateGauntlets
            | ItemType::PlateHelmet
            | ItemType::Scroll
            | ItemType::Shirt
            | ItemType::Shackles
            | ItemType::TopHat
            | ItemType::Trousers
            | ItemType::Vest => return None,
        };

        let level = self.level();
        let level_modifier = if level == 1 {
            0
        } else if level >= 2 && level <= 10 {
            2
        } else if level >= 11 && level <= 20 {
            4
        } else if level >= 21 && level <= 30 {
            6
        } else {
            10
        };

        Some(base_modifier + level_modifier)
    }

    fn resistance(&self) -> Option<i32> {
        let base_resistance = match self.item_type {
            ItemType::Boots
            | ItemType::Buckler
            | ItemType::Shield
            | ItemType::Vest
            | ItemType::Helm => 2,
            ItemType::Shirt
            | ItemType::Gloves
            | ItemType::Trousers
            | ItemType::Cloak
            | ItemType::LoinCloth
            | ItemType::Shackles
            | ItemType::BowlerHat
            | ItemType::Crown
            | ItemType::Fedora
            | ItemType::Mask
            | ItemType::TopHat => 0,
            ItemType::Breastplate
            | ItemType::PlateBoots
            | ItemType::PlateGauntlets
            | ItemType::PlateHelmet => 4,
            ItemType::Club
            | ItemType::Dagger
            | ItemType::Dirk
            | ItemType::GreatSword
            | ItemType::Halberd
            | ItemType::Hammer
            | ItemType::LongSword
            | ItemType::Mace
            | ItemType::Morningstar
            | ItemType::Pike
            | ItemType::Scroll
            | ItemType::ShortSword
            | ItemType::Spear
            | ItemType::Whip => return None,
        };

        let level = self.level();
        let level_modifier = if level == 1 {
            0
        } else if level >= 2 && level <= 10 {
            1
        } else if level >= 11 && level <= 20 {
            3
        } else if level >= 21 && level <= 30 {
            5
        } else {
            8
        };

        Some(base_resistance + level_modifier)
    }

    fn level(&self) -> u32 {
        self.item_level.unwrap_or(1)
    }
}
