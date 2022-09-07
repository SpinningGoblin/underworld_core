use rand::{prelude::ThreadRng, Rng};
use std::ops::RangeInclusive;
use uuid::Uuid;

use crate::components::{
    damage::AttackEffect,
    items::{Descriptor, Item, ItemType},
    Material, Tagged, {Attack, Defense},
};

use super::generator::Generator;

pub fn item_generator_for_level(
    item_type: &ItemType,
    is_equipped: bool,
    level: u32,
) -> impl Generator<Item> {
    ItemPrototype {
        item_type: *item_type,
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
        let mut rng = rand::thread_rng();
        let material = self.material();
        let descriptors = self.descriptors(&material);
        let attack = self.attack(&mut rng);
        let defense = self.defense(&mut rng);
        let tags = self.item_type.tags();

        Item {
            id: Uuid::new_v4(),
            name: None,
            item_type: self.item_type,
            tags,
            descriptors,
            material,
            attack,
            defense,
            consumable: None,
            throwable: None,
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

        let mut possible_descriptors: Vec<Descriptor> =
            super::utils::item_descriptors::possible_descriptors(
                &self.item_type,
                material,
                self.level(),
            );
        let descriptors = num_descriptor_range.filter_map(|_| {
            if possible_descriptors.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_descriptors.len());
                Some(possible_descriptors.remove(index))
            }
        });

        descriptors.into_iter().collect()
    }

    fn attack(&self, rng: &mut ThreadRng) -> Option<Attack> {
        let base_rolls = match self.item_type {
            ItemType::Buckler
            | ItemType::Dagger
            | ItemType::Dirk
            | ItemType::Shield
            | ItemType::ShortSword => 1,
            ItemType::Club
            | ItemType::Hammer
            | ItemType::LongSword
            | ItemType::Mace
            | ItemType::Morningstar
            | ItemType::Whip
            | ItemType::Spear
            | ItemType::Pike => 2,
            ItemType::GreatSword | ItemType::Halberd => 3,
            ItemType::Breastplate
            | ItemType::Boots
            | ItemType::BowlerHat
            | ItemType::Cloak
            | ItemType::Crown
            | ItemType::Fedora
            | ItemType::Flask
            | ItemType::Gloves
            | ItemType::Helm
            | ItemType::LoinCloth
            | ItemType::Mask
            | ItemType::PlateBoots
            | ItemType::PlateGauntlets
            | ItemType::PlateHelmet
            | ItemType::Pot
            | ItemType::Scroll
            | ItemType::Shirt
            | ItemType::Shackles
            | ItemType::TopHat
            | ItemType::Trousers
            | ItemType::Vest => return None,
        };

        let level = self.level();
        let (additional_rolls, modifier) = if level == 1 {
            (0, -1)
        } else if (2..=5).contains(&level) {
            (0, rng.gen_range(0..=2))
        } else if (6..=10).contains(&level) {
            (1, 0)
        } else if (11..=15).contains(&level) {
            (1, rng.gen_range(0..=2))
        } else if (16..=20).contains(&level) {
            (2, 0)
        } else if (21..=25).contains(&level) {
            (2, rng.gen_range(0..=2))
        } else if (26..=30).contains(&level) {
            (3, 0)
        } else if (31..=35).contains(&level) {
            (3, rng.gen_range(0..=2))
        } else if (36..=40).contains(&level) {
            (4, 0)
        } else if (41..=45).contains(&level) {
            (4, rng.gen_range(0..=2))
        } else if (46..=50).contains(&level) {
            (5, 1)
        } else if (51..=55).contains(&level) {
            (5, rng.gen_range(0..=2))
        } else {
            (7, 1)
        };

        let max_rolls = base_rolls + additional_rolls;
        let roll_range = base_rolls..=max_rolls;

        let mut rng = rand::thread_rng();
        let num_rolls = rng.gen_range(roll_range);

        let (num_effects, mut possible_effects) = if (1..=30).contains(&level) {
            (0, Vec::new())
        } else if (31..=50).contains(&level) {
            (
                rng.gen_range(0..=1),
                vec![AttackEffect::Crushing, AttackEffect::Sharp],
            )
        } else if (51..=65).contains(&level) {
            (
                rng.gen_range(0..=1),
                vec![
                    AttackEffect::Crushing,
                    AttackEffect::Sharp,
                    AttackEffect::Acidic,
                    AttackEffect::Toxic,
                ],
            )
        } else {
            (
                rng.gen_range(1..=3),
                vec![
                    AttackEffect::Crushing,
                    AttackEffect::Sharp,
                    AttackEffect::Acidic,
                    AttackEffect::Toxic,
                ],
            )
        };

        let effects: Vec<AttackEffect> = if num_effects > 0 {
            (0..num_effects)
                .map(|_| {
                    let index = rng.gen_range(0..possible_effects.len());
                    possible_effects.remove(index)
                })
                .collect()
        } else {
            Vec::new()
        };

        Some(Attack {
            num_rolls,
            modifier,
            effects,
        })
    }

    fn defense(&self, rng: &mut ThreadRng) -> Option<Defense> {
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
            | ItemType::Flask
            | ItemType::GreatSword
            | ItemType::Halberd
            | ItemType::Hammer
            | ItemType::LongSword
            | ItemType::Mace
            | ItemType::Morningstar
            | ItemType::Pike
            | ItemType::Pot
            | ItemType::Scroll
            | ItemType::ShortSword
            | ItemType::Spear
            | ItemType::Whip => return None,
        };

        let level = self.level();
        let level_modifier = if level == 1 {
            0
        } else if (2..=10).contains(&level) {
            rng.gen_range(1..=3)
        } else if (11..=20).contains(&level) {
            rng.gen_range(2..=4)
        } else if (21..=30).contains(&level) {
            rng.gen_range(4..=6)
        } else if (31..=40).contains(&level) {
            rng.gen_range(7..=9)
        } else {
            rng.gen_range(10..=25)
        };

        Some(Defense {
            damage_resistance: base_resistance + level_modifier,
        })
    }

    fn level(&self) -> u32 {
        self.item_level.unwrap_or(1)
    }
}
