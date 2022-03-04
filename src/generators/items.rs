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
        materials: possible_materials(item_type),
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
        let defense = self.num_defense_rolls().map(|num_rolls| Defense {
            num_rolls,
            modifier: self.defense_modifier().unwrap_or_default(),
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
            ItemType::GreatSword | ItemType::Hammer | ItemType::LongSword => Some(1),
            _ => None,
        }
    }

    fn defense_modifier(&self) -> Option<i32> {
        match self.item_type {
            ItemType::Boots
            | ItemType::Buckler
            | ItemType::Cloak
            | ItemType::Shield
            | ItemType::Shirt
            | ItemType::Trousers
            | ItemType::Vest => Some(-1),
            ItemType::Crown | ItemType::Gloves | ItemType::Mask => Some(-2),
            ItemType::LoinCloth | ItemType::Shackles => Some(-3),
            _ => None,
        }
    }

    fn num_defense_rolls(&self) -> Option<usize> {
        match self.item_type {
            ItemType::Breastplate
            | ItemType::Boots
            | ItemType::Buckler
            | ItemType::Helm
            | ItemType::PlateBoots
            | ItemType::PlateGauntlets
            | ItemType::PlateHelmet
            | ItemType::Shield
            | ItemType::Cloak
            | ItemType::Crown
            | ItemType::Gloves
            | ItemType::LoinCloth
            | ItemType::Mask
            | ItemType::Shirt
            | ItemType::Shackles
            | ItemType::Trousers
            | ItemType::Vest => Some(1),
            _ => None,
        }
    }
}

fn possible_materials(item_type: &ItemType) -> Vec<Material> {
    match *item_type {
        ItemType::Breastplate => vec![Material::Iron, Material::Leather, Material::Steel],
        ItemType::Mask => vec![Material::Bone, Material::Iron],
        ItemType::Cloak => {
            vec![Material::Linen, Material::Hide, Material::Wool]
        }
        ItemType::Shirt => vec![
            Material::Wool,
            Material::Linen,
            Material::Cotton,
            Material::Silk,
        ],
        ItemType::Trousers => vec![
            Material::Hide,
            Material::Leather,
            Material::Wool,
            Material::Linen,
        ],
        ItemType::Crown => {
            vec![Material::Bone, Material::Gold, Material::Stone]
        }
        ItemType::Boots => vec![
            Material::Hide,
            Material::Iron,
            Material::Leather,
            Material::Steel,
        ],
        ItemType::Gloves | ItemType::Vest => vec![Material::Fur, Material::Hide, Material::Leather],
        ItemType::LoinCloth => vec![
            Material::Hide,
            Material::Wool,
            Material::Leather,
            Material::Silk,
            Material::Linen,
            Material::Cotton,
        ],
        ItemType::PlateBoots | ItemType::PlateGauntlets | ItemType::PlateHelmet => {
            vec![Material::Iron, Material::Steel]
        }
        ItemType::Shackles => vec![Material::Iron, Material::Leather, Material::Steel],
        ItemType::Buckler => {
            vec![Material::Hide, Material::Iron, Material::Steel]
        }
        ItemType::Club => vec![Material::Bone, Material::Stone, Material::Wooden],
        ItemType::Dagger => vec![
            Material::Bone,
            Material::Gold,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::Dirk | ItemType::GreatSword => vec![
            Material::Bone,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::Hammer | ItemType::LongSword => {
            vec![Material::Bone, Material::Iron, Material::Steel]
        }
        ItemType::Mace => vec![Material::Iron, Material::Steel],
        ItemType::Morningstar => vec![Material::Iron, Material::Steel],
        ItemType::Shield => vec![
            Material::Hide,
            Material::Iron,
            Material::Leather,
            Material::Steel,
            Material::Wooden,
        ],
        ItemType::ShortSword => vec![Material::Iron, Material::Steel],
        ItemType::Whip => vec![Material::Leather],
        ItemType::Helm => vec![
            Material::Iron,
            Material::Hide,
            Material::Steel,
            Material::Leather,
            Material::Fur,
        ],
        ItemType::Halberd => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
        ItemType::Pike => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
        ItemType::Spear => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
    }
}
