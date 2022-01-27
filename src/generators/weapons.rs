use std::ops::Range;

use rand::Rng;

use crate::components::{
    attack::Attack,
    weapon::{Weapon, WeaponDescriptor, WeaponMaterial, WeaponType},
};

use super::generator::Generator;

pub struct WeaponGenerator;

impl WeaponGenerator {
    pub fn for_weapon_type(weapon_type: &WeaponType) -> impl Generator<Weapon> {
        match *weapon_type {
            WeaponType::Club => WeaponPrototype::club(),
            WeaponType::Dagger => WeaponPrototype::dagger(),
            WeaponType::Hammer => WeaponPrototype::hammer(),
            WeaponType::LongSword => WeaponPrototype::long_sword(),
            WeaponType::ShortSword => WeaponPrototype::short_sword(),
            WeaponType::Buckler => WeaponPrototype::buckler(),
            WeaponType::Dirk => WeaponPrototype::dirk(),
            WeaponType::GreatSword => WeaponPrototype::great_sword(),
            WeaponType::Mace => WeaponPrototype::mace(),
            WeaponType::Morningstar => WeaponPrototype::morningstar(),
            WeaponType::Shield => WeaponPrototype::shield(),
            WeaponType::Whip => WeaponPrototype::whip(),
        }
    }
}

struct WeaponPrototype {
    pub weapon_type: WeaponType,
    pub num_descriptors: Range<usize>,
    pub attack: Option<Attack>,
    pub possible_materials: Vec<WeaponMaterial>,
}

impl WeaponPrototype {
    pub fn buckler() -> Self {
        Self {
            weapon_type: WeaponType::Buckler,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 0,
                maximum: 2,
            }),
            possible_materials: vec![WeaponMaterial::Steel, WeaponMaterial::Iron],
        }
    }

    pub fn dagger() -> Self {
        Self {
            weapon_type: WeaponType::Dagger,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn dirk() -> Self {
        Self {
            weapon_type: WeaponType::Dirk,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn club() -> Self {
        Self {
            weapon_type: WeaponType::Club,
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Stone,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn great_sword() -> Self {
        Self {
            weapon_type: WeaponType::LongSword,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 12,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn hammer() -> Self {
        Self {
            weapon_type: WeaponType::Hammer,
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn long_sword() -> Self {
        Self {
            weapon_type: WeaponType::LongSword,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 6,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn mace() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 6,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn morningstar() -> Self {
        Self {
            weapon_type: WeaponType::Morningstar,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 6,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn short_sword() -> Self {
        Self {
            weapon_type: WeaponType::ShortSword,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 4,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Stone,
                WeaponMaterial::Gold,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn shield() -> Self {
        Self {
            weapon_type: WeaponType::Shield,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 4,
            }),
            possible_materials: vec![
                WeaponMaterial::Bone,
                WeaponMaterial::Steel,
                WeaponMaterial::Wooden,
                WeaponMaterial::Iron,
            ],
        }
    }

    pub fn whip() -> Self {
        Self {
            weapon_type: WeaponType::Whip,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 6,
            }),
            possible_materials: vec![WeaponMaterial::Steel, WeaponMaterial::Leather],
        }
    }
}

impl Generator<Weapon> for WeaponPrototype {
    fn generate(&self) -> Weapon {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let mut possible_descriptors: Vec<WeaponDescriptor> =
            self.weapon_type.possible_descriptors().to_vec();
        let mut descriptors: Vec<WeaponDescriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        let material = if !self.possible_materials.is_empty() {
            let index = rng.gen_range(0..self.possible_materials.len());
            self.possible_materials.get(index).cloned()
        } else {
            None
        };

        Weapon {
            attack: self.attack.clone(),
            weapon_type: self.weapon_type.clone(),
            descriptors,
            material,
        }
    }
}
