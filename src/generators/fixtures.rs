use std::ops::RangeInclusive;

use rand::{prelude::ThreadRng, Rng};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    components::{
        fixtures::{Fixture, FixtureType},
        items::{Descriptor, FixtureItem, Item, ItemType},
        BuiltWithMaterial, Size, {Tag, Tagged},
    },
    utils::rolls::roll_percent_succeeds,
};

use super::{
    generator::Generator, items::item_generator_for_level, utils::item_descriptors::matches_tags,
};

const HAS_MATERIAL_CHANCE: i32 = 90;
const HAS_NON_STANDARD_SIZE: i32 = 50;

pub struct FixturePrototype {
    pub fixture_type: FixtureType,
    pub num_items: RangeInclusive<usize>,
    pub num_hidden_items: RangeInclusive<usize>,
    pub has_hidden_compartment: bool,
    pub danger_level: u32,
}

pub fn get_generator(
    fixture_type: &FixtureType,
    has_hidden_compartment: bool,
) -> impl Generator<Fixture> {
    FixturePrototype {
        fixture_type: *fixture_type,
        has_hidden_compartment,
        num_items: 0..=2,
        num_hidden_items: 0..=2,
        danger_level: 1,
    }
}

pub fn get_generator_for_level(
    fixture_type: &FixtureType,
    has_hidden_compartment: bool,
    danger_level: u32,
) -> impl Generator<Fixture> {
    FixturePrototype {
        danger_level,
        fixture_type: *fixture_type,
        has_hidden_compartment,
        num_items: 0..=2,
        num_hidden_items: 0..=2,
    }
}

impl Generator<Fixture> for FixturePrototype {
    fn generate(&self) -> Fixture {
        let mut rng = rand::thread_rng();
        let has_material = roll_percent_succeeds(&mut rng, HAS_MATERIAL_CHANCE);

        let material = if has_material {
            let possible_materials = self.fixture_type.possible_materials();
            if possible_materials.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_materials.len());
                possible_materials.get(index).cloned()
            }
        } else {
            None
        };

        let size = if roll_percent_succeeds(&mut rng, HAS_NON_STANDARD_SIZE) {
            let possibilities = non_average_sizes();
            if possibilities.is_empty() {
                Size::Average
            } else {
                let index = rng.gen_range(0..possibilities.len());
                match possibilities.get(index) {
                    Some(height) => *height,
                    None => Size::Average,
                }
            }
        } else {
            Size::Average
        };

        let mut num_descriptors = rng.gen_range(0..=2);
        let mut possible_descriptors: Vec<Descriptor> = match &material {
            Some(material) => {
                let tags: Vec<Tag> = self
                    .fixture_type
                    .tags()
                    .into_iter()
                    .chain(material.tags().into_iter())
                    .collect();
                matches_tags(&tags)
            }
            None => matches_tags(&self.fixture_type.tags()),
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

        let items: Vec<FixtureItem> = if fixture_can_have_items(&self.fixture_type) {
            let num_items = rng.gen_range(self.num_items.clone());
            build_items(
                &self.fixture_type,
                num_items,
                &size,
                &mut rng,
                self.danger_level,
            )
            .into_iter()
            .map(|item| FixtureItem {
                item,
                is_inside: items_go_inside(&self.fixture_type),
                is_in_hidden_compartment: false,
            })
            .collect()
        } else {
            Vec::new()
        };

        let hidden_compartment_items: Vec<FixtureItem> = if self.has_hidden_compartment {
            let num_items = rng.gen_range(self.num_hidden_items.clone());
            build_items(
                &self.fixture_type,
                num_items,
                &size,
                &mut rng,
                self.danger_level,
            )
            .into_iter()
            .map(|item| FixtureItem {
                item,
                is_inside: false,
                is_in_hidden_compartment: true,
            })
            .collect()
        } else {
            Vec::new()
        };

        Fixture {
            id: Uuid::new_v4(),
            name: None,
            material,
            fixture_type: self.fixture_type,
            size,
            descriptors,
            items: items
                .into_iter()
                .chain(hidden_compartment_items.into_iter())
                .collect(),
            has_hidden_compartment: self.has_hidden_compartment,
            can_be_opened: fixture_can_be_opened(&self.fixture_type),
            open: false,
            hidden_compartment_open: false,
        }
    }
}

fn items_go_inside(fixture_type: &FixtureType) -> bool {
    match *fixture_type {
        FixtureType::Barrel | FixtureType::Chest | FixtureType::Coffin | FixtureType::Crate => true,
        FixtureType::Bed
        | FixtureType::Bucket
        | FixtureType::Chair
        | FixtureType::Cot
        | FixtureType::Pillar
        | FixtureType::SleepingRoll
        | FixtureType::StatueTentacledMonstrosity
        | FixtureType::StatueWarrior
        | FixtureType::Table
        | FixtureType::WeaponRack => false,
    }
}

fn fixture_can_be_opened(fixture_type: &FixtureType) -> bool {
    match *fixture_type {
        FixtureType::Barrel
        | FixtureType::Chest
        | FixtureType::Coffin
        | FixtureType::WeaponRack => true,
        FixtureType::Bed
        | FixtureType::Bucket
        | FixtureType::Chair
        | FixtureType::Cot
        | FixtureType::Crate
        | FixtureType::Pillar
        | FixtureType::SleepingRoll
        | FixtureType::StatueTentacledMonstrosity
        | FixtureType::StatueWarrior
        | FixtureType::Table => false,
    }
}

fn build_items(
    fixture_type: &FixtureType,
    num_items: usize,
    size: &Size,
    rng: &mut ThreadRng,
    danger_level: u32,
) -> Vec<Item> {
    if num_items == 0 {
        return Vec::new();
    }

    let range = 0..num_items;
    let item_types = possible_item_types(fixture_type, size);
    range
        .flat_map(|_| {
            if item_types.is_empty() {
                None
            } else {
                let item_type_index = rng.gen_range(0..item_types.len());
                match item_types.get(item_type_index) {
                    Some(item_type) => {
                        let generator = item_generator_for_level(item_type, false, danger_level);
                        Some(generator.generate())
                    }
                    None => None,
                }
            }
        })
        .collect()
}

fn fixture_can_have_items(fixture_type: &FixtureType) -> bool {
    match *fixture_type {
        FixtureType::Barrel
        | FixtureType::Bed
        | FixtureType::Bucket
        | FixtureType::Chest
        | FixtureType::Coffin
        | FixtureType::WeaponRack
        | FixtureType::Table
        | FixtureType::Crate
        | FixtureType::Chair
        | FixtureType::Cot
        | FixtureType::SleepingRoll => true,
        FixtureType::Pillar
        | FixtureType::StatueTentacledMonstrosity
        | FixtureType::StatueWarrior => false,
    }
}

fn possible_item_types(fixture_type: &FixtureType, size: &Size) -> Vec<ItemType> {
    match (fixture_type, size) {
        (FixtureType::Barrel, Size::Small) => vec![
            ItemType::Dagger,
            ItemType::Crown,
            ItemType::Dirk,
            ItemType::Whip,
            ItemType::Trousers,
            ItemType::PlateGauntlets,
            ItemType::PlateHelmet,
            ItemType::Shackles,
            ItemType::Shirt,
            ItemType::ShortSword,
            ItemType::Gloves,
            ItemType::Hammer,
            ItemType::LoinCloth,
            ItemType::Cloak,
            ItemType::Vest,
            ItemType::Boots,
            ItemType::Buckler,
            ItemType::Mace,
            ItemType::Mask,
            ItemType::Morningstar,
        ],
        (FixtureType::Barrel, Size::Tiny) => vec![ItemType::Dagger, ItemType::LoinCloth],
        (FixtureType::Bed, Size::Small) => vec![
            ItemType::LoinCloth,
            ItemType::Shirt,
            ItemType::Shackles,
            ItemType::Crown,
            ItemType::Mask,
            ItemType::Mace,
        ],
        (FixtureType::Bed, Size::Tiny) => vec![ItemType::LoinCloth],
        (FixtureType::Bucket, Size::Average) => vec![
            ItemType::LoinCloth,
            ItemType::Dagger,
            ItemType::Shackles,
            ItemType::Crown,
        ],
        (FixtureType::Bucket, Size::Huge) => vec![
            ItemType::LoinCloth,
            ItemType::Dagger,
            ItemType::Shackles,
            ItemType::Crown,
            ItemType::Shirt,
            ItemType::ShortSword,
        ],
        (FixtureType::Bucket, Size::Large) => vec![
            ItemType::LoinCloth,
            ItemType::Dagger,
            ItemType::Shackles,
            ItemType::Crown,
            ItemType::Shirt,
            ItemType::ShortSword,
        ],
        (FixtureType::Bucket, Size::Massive) => vec![
            ItemType::LoinCloth,
            ItemType::Dagger,
            ItemType::Shackles,
            ItemType::Crown,
            ItemType::Shirt,
            ItemType::ShortSword,
        ],
        (FixtureType::Bucket, Size::Medium) => vec![
            ItemType::LoinCloth,
            ItemType::Dagger,
            ItemType::Shackles,
            ItemType::Crown,
        ],
        (FixtureType::Bucket, Size::Small) => vec![ItemType::LoinCloth],
        (FixtureType::Bucket, Size::Tiny) => vec![ItemType::Crown],
        (FixtureType::Chest, Size::Average) => vec![
            ItemType::Dagger,
            ItemType::Crown,
            ItemType::Dirk,
            ItemType::Whip,
            ItemType::Trousers,
            ItemType::PlateGauntlets,
            ItemType::PlateHelmet,
            ItemType::Shackles,
            ItemType::Shirt,
            ItemType::ShortSword,
            ItemType::Gloves,
            ItemType::Hammer,
            ItemType::LoinCloth,
            ItemType::Cloak,
            ItemType::Vest,
            ItemType::Boots,
            ItemType::Buckler,
            ItemType::Mace,
            ItemType::Mask,
            ItemType::Morningstar,
        ],
        (FixtureType::Chest, Size::Small) => {
            vec![ItemType::Crown, ItemType::Mask, ItemType::ShortSword]
        }
        (FixtureType::Chest, Size::Tiny) => {
            vec![ItemType::Crown, ItemType::Mask, ItemType::ShortSword]
        }
        (FixtureType::Barrel, Size::Average)
        | (FixtureType::Barrel, Size::Huge)
        | (FixtureType::Barrel, Size::Large)
        | (FixtureType::Barrel, Size::Massive)
        | (FixtureType::Barrel, Size::Medium)
        | (FixtureType::Bed, Size::Average)
        | (FixtureType::Bed, Size::Huge)
        | (FixtureType::Bed, Size::Large)
        | (FixtureType::Bed, Size::Massive)
        | (FixtureType::Chest, Size::Huge)
        | (FixtureType::Chest, Size::Large)
        | (FixtureType::Chest, Size::Massive)
        | (FixtureType::Coffin, Size::Average)
        | (FixtureType::Coffin, Size::Huge)
        | (FixtureType::Coffin, Size::Large)
        | (FixtureType::Coffin, Size::Massive)
        | (FixtureType::Coffin, Size::Small)
        | (FixtureType::Coffin, Size::Tiny)
        | (FixtureType::Crate, Size::Average)
        | (FixtureType::Crate, Size::Huge)
        | (FixtureType::Crate, Size::Large)
        | (FixtureType::Crate, Size::Massive)
        | (FixtureType::Crate, Size::Medium)
        | (FixtureType::Crate, Size::Small)
        | (FixtureType::Crate, Size::Tiny)
        | (FixtureType::Pillar, Size::Average)
        | (FixtureType::Pillar, Size::Huge)
        | (FixtureType::Pillar, Size::Large)
        | (FixtureType::Pillar, Size::Massive)
        | (FixtureType::Pillar, Size::Medium)
        | (FixtureType::Pillar, Size::Small)
        | (FixtureType::Pillar, Size::Tall)
        | (FixtureType::Pillar, Size::Tiny)
        | (FixtureType::StatueTentacledMonstrosity, Size::Average)
        | (FixtureType::StatueTentacledMonstrosity, Size::Huge)
        | (FixtureType::StatueTentacledMonstrosity, Size::Large)
        | (FixtureType::StatueTentacledMonstrosity, Size::Massive)
        | (FixtureType::StatueTentacledMonstrosity, Size::Medium)
        | (FixtureType::StatueTentacledMonstrosity, Size::Short)
        | (FixtureType::StatueTentacledMonstrosity, Size::Small)
        | (FixtureType::StatueTentacledMonstrosity, Size::Squat)
        | (FixtureType::StatueTentacledMonstrosity, Size::Tall)
        | (FixtureType::StatueTentacledMonstrosity, Size::Tiny)
        | (FixtureType::StatueWarrior, Size::Average)
        | (FixtureType::StatueWarrior, Size::Huge)
        | (FixtureType::StatueWarrior, Size::Large)
        | (FixtureType::StatueWarrior, Size::Massive)
        | (FixtureType::StatueWarrior, Size::Medium)
        | (FixtureType::StatueWarrior, Size::Short)
        | (FixtureType::StatueWarrior, Size::Small)
        | (FixtureType::StatueWarrior, Size::Squat)
        | (FixtureType::StatueWarrior, Size::Tall)
        | (FixtureType::StatueWarrior, Size::Tiny)
        | (FixtureType::Table, Size::Average)
        | (FixtureType::Table, Size::Huge)
        | (FixtureType::Table, Size::Large)
        | (FixtureType::Table, Size::Massive)
        | (FixtureType::Table, Size::Long)
        | (FixtureType::Table, Size::Medium)
        | (FixtureType::Table, Size::Narrow)
        | (FixtureType::Table, Size::Short)
        | (FixtureType::Table, Size::Small)
        | (FixtureType::Table, Size::Squat)
        | (FixtureType::Table, Size::Tall)
        | (FixtureType::Table, Size::Tiny)
        | (FixtureType::Table, Size::Wide)
        | (FixtureType::WeaponRack, Size::Average)
        | (FixtureType::WeaponRack, Size::Huge)
        | (FixtureType::WeaponRack, Size::Large)
        | (FixtureType::WeaponRack, Size::Massive)
        | (FixtureType::WeaponRack, Size::Long)
        | (FixtureType::WeaponRack, Size::Medium)
        | (FixtureType::WeaponRack, Size::Narrow)
        | (FixtureType::WeaponRack, Size::Short)
        | (FixtureType::WeaponRack, Size::Small)
        | (FixtureType::WeaponRack, Size::Squat)
        | (FixtureType::WeaponRack, Size::Tall)
        | (FixtureType::WeaponRack, Size::Tiny)
        | (FixtureType::WeaponRack, Size::Wide) => ItemType::iter()
            .filter(|item_type| !matches!(item_type, ItemType::Scroll))
            .collect(),
        _ => Vec::new(),
    }
}

fn non_average_sizes() -> Vec<Size> {
    vec![
        Size::Small,
        Size::Tiny,
        Size::Large,
        Size::Huge,
        Size::Massive,
    ]
}
