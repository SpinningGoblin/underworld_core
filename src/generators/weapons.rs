use std::ops::Range;

use rand::Rng;

use crate::components::{
    attack::Attack,
    item_descriptor::ItemDescriptor,
    item_material::{BuiltWithMaterial, ItemMaterial},
    weapons::{weapon::Weapon, weapon_type::WeaponType},
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
    pub possible_materials: Vec<ItemMaterial>,
}

impl WeaponPrototype {
    pub fn build(
        weapon_type: WeaponType,
        num_descriptors: Range<usize>,
        attack: Option<Attack>,
    ) -> Self {
        let possible_materials = weapon_type.possible_materials();
        Self {
            weapon_type,
            num_descriptors,
            attack,
            possible_materials,
        }
    }

    pub fn buckler() -> Self {
        Self::build(
            WeaponType::Buckler,
            0..3,
            Some(Attack {
                minimum: 0,
                maximum: 2,
            }),
        )
    }

    pub fn dagger() -> Self {
        Self::build(
            WeaponType::Dagger,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn dirk() -> Self {
        Self::build(
            WeaponType::Dirk,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn club() -> Self {
        Self::build(
            WeaponType::Club,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn great_sword() -> Self {
        Self::build(
            WeaponType::GreatSword,
            0..3,
            Some(Attack {
                minimum: 3,
                maximum: 6,
            }),
        )
    }

    pub fn hammer() -> Self {
        Self::build(
            WeaponType::Hammer,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn long_sword() -> Self {
        Self::build(
            WeaponType::LongSword,
            0..3,
            Some(Attack {
                minimum: 2,
                maximum: 4,
            }),
        )
    }

    pub fn mace() -> Self {
        Self::build(
            WeaponType::Mace,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn morningstar() -> Self {
        Self::build(
            WeaponType::Morningstar,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn short_sword() -> Self {
        Self::build(
            WeaponType::ShortSword,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn shield() -> Self {
        Self::build(
            WeaponType::Shield,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn whip() -> Self {
        Self::build(
            WeaponType::Whip,
            0..3,
            Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        )
    }
}

impl Generator<Weapon> for WeaponPrototype {
    fn generate(&self) -> Weapon {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let material = if !self.possible_materials.is_empty() {
            let index = rng.gen_range(0..self.possible_materials.len());
            self.possible_materials.get(index).cloned()
        } else {
            None
        };

        let mut possible_descriptors: Vec<ItemDescriptor> = match &material {
            Some(material) => ItemDescriptor::matches_two_tagged(&self.weapon_type, material),
            None => ItemDescriptor::matches_tagged(&self.weapon_type),
        };
        let mut descriptors: Vec<ItemDescriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        Weapon {
            attack: self.attack.clone(),
            weapon_type: self.weapon_type.clone(),
            descriptors,
            material,
        }
    }
}
