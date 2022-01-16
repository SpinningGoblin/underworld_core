use std::ops::Range;

use rand::Rng;

use crate::components::{
    attack::Attack,
    weapon::{Weapon, WeaponDescriptor, WeaponType},
};

use super::generator::Generator;

pub struct WeaponPrototype {
    pub weapon_type: WeaponType,
    pub possible_descriptors: Vec<WeaponDescriptor>,
    pub num_descriptors: Range<usize>,
    pub attack: Option<Attack>,
}

impl WeaponPrototype {
    pub fn dagger() -> Self {
        Self {
            weapon_type: WeaponType::Dagger,
            possible_descriptors: vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn club() -> Self {
        Self {
            weapon_type: WeaponType::Club,
            possible_descriptors: vec![WeaponDescriptor::Broken],
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn hammer() -> Self {
        Self {
            weapon_type: WeaponType::Hammer,
            possible_descriptors: vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Rusty,
            ],
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn long_sword() -> Self {
        Self {
            weapon_type: WeaponType::LongSword,
            possible_descriptors: vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 6,
            }),
        }
    }

    pub fn short_sword() -> Self {
        Self {
            weapon_type: WeaponType::ShortSword,
            possible_descriptors: vec![
                WeaponDescriptor::Broken,
                WeaponDescriptor::Chipped,
                WeaponDescriptor::Dull,
                WeaponDescriptor::Rusty,
                WeaponDescriptor::Shiny,
            ],
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 4,
            }),
        }
    }
}

impl Generator<Weapon> for WeaponPrototype {
    fn generate(&self) -> Weapon {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let mut possible_descriptors: Vec<WeaponDescriptor> =
            self.possible_descriptors.to_vec();
        let mut descriptors: Vec<WeaponDescriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.insert(0, descriptor);

            num_descriptors -= 1;
        }

        Weapon {
            attack: self.attack.clone(),
            weapon_type: self.weapon_type.clone(),
            descriptors,
        }
    }
}
