use std::ops::RangeInclusive;

use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::components::{
    items::{descriptor::Descriptor, item::Item, item_type::ItemType},
    material::Material,
    tag::Tagged,
};

use super::generator::Generator;

pub fn random_item_generator() -> impl Generator<Item> {
    let mut rng = rand::thread_rng();
    let item_types: Vec<ItemType> = ItemType::into_enum_iter().collect();
    let index = rng.gen_range(0..item_types.len());
    let item_type = item_types.get(index).unwrap();
    let materials = possible_materials(item_type);
    ItemPrototype {
        materials,
        item_type: item_type.clone(),
        num_descriptors: 1..=2,
    }
}

pub fn item_generator(item_type: &ItemType) -> impl Generator<Item> {
    ItemPrototype {
        item_type: item_type.clone(),
        num_descriptors: 1..=2,
        materials: possible_materials(item_type),
    }
}

pub struct ItemPrototype {
    pub item_type: ItemType,
    pub num_descriptors: RangeInclusive<usize>,
    pub materials: Vec<Material>,
}

impl Generator<Item> for ItemPrototype {
    fn generate(&self) -> Item {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let material = if !self.materials.is_empty() {
            let index = rng.gen_range(0..self.materials.len());
            self.materials.get(index).cloned()
        } else {
            None
        };

        let mut possible_descriptors: Vec<Descriptor> = match &material {
            Some(material) => Descriptor::matches_two_tagged(&self.item_type, material),
            None => Descriptor::matches_tagged(&self.item_type),
        };
        let mut descriptors: Vec<Descriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        let tags = self.item_type.tags();

        Item {
            attack: None,
            item_type: self.item_type.clone(),
            descriptors,
            material,
            tags,
            defense: None,
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
        ItemType::Gloves => vec![Material::Hide, Material::Leather],
        ItemType::LoinCloth => vec![
            Material::Hide,
            Material::Wool,
            Material::Leather,
            Material::Silk,
            Material::Linen,
            Material::Cotton,
        ],
        ItemType::PlateBoots => vec![Material::Iron, Material::Steel],
        ItemType::PlateGauntlets => vec![Material::Iron, Material::Steel],
        ItemType::PlateHelmet => vec![Material::Iron, Material::Steel],
        ItemType::Shackles => vec![Material::Iron, Material::Leather, Material::Steel],
        ItemType::Vest => {
            vec![Material::Fur, Material::Hide, Material::Leather]
        }
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
        ItemType::Dirk => vec![
            Material::Bone,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::GreatSword => vec![
            Material::Bone,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::Hammer => vec![Material::Iron, Material::Steel],
        ItemType::LongSword => {
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
    }
}
