use std::ops::Range;

use rand::Rng;

use crate::components::{
    defense::Defense,
    wearable::{Wearable, WearableDescriptor, WearableMaterial, WearableType},
};

use super::generator::Generator;

struct WearablePrototype {
    pub wearable_type: WearableType,
    pub num_descriptors: Range<usize>,
    pub defense: Option<Defense>,
    pub possible_materials: Vec<WearableMaterial>,
}

pub struct WearableGenerator;

impl WearableGenerator {
    pub fn for_wearable_type(wearable_type: &WearableType) -> impl Generator<Wearable> {
        match *wearable_type {
            WearableType::Armour => WearablePrototype::armour(),
            WearableType::Cloak => WearablePrototype::cloak(),
            WearableType::Clothing => WearablePrototype::clothing(),
            WearableType::PlateMailHelmet => WearablePrototype::plate_mail(),
            WearableType::Shackles => WearablePrototype::shackles(),
        }
    }
}

impl WearablePrototype {
    pub fn armour() -> Self {
        Self {
            wearable_type: WearableType::Armour,
            num_descriptors: 0..3,
            defense: Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
            possible_materials: vec![
                WearableMaterial::Iron,
                WearableMaterial::Leather,
                WearableMaterial::Steel,
            ],
        }
    }

    pub fn cloak() -> Self {
        Self {
            wearable_type: WearableType::Cloak,
            num_descriptors: 0..3,
            defense: Some(Defense {
                minimum: 0,
                maximum: 2,
            }),
            possible_materials: Vec::new(),
        }
    }

    pub fn clothing() -> Self {
        Self {
            wearable_type: WearableType::Clothing,
            num_descriptors: 0..3,
            defense: Some(Defense {
                minimum: 0,
                maximum: 1,
            }),
            possible_materials: Vec::new(),
        }
    }

    pub fn plate_mail() -> Self {
        Self {
            wearable_type: WearableType::PlateMailHelmet,
            num_descriptors: 0..3,
            defense: Some(Defense {
                minimum: 3,
                maximum: 6,
            }),
            possible_materials: vec![WearableMaterial::Iron, WearableMaterial::Steel],
        }
    }

    pub fn shackles() -> Self {
        Self {
            wearable_type: WearableType::Shackles,
            num_descriptors: 0..2,
            defense: None,
            possible_materials: vec![WearableMaterial::Iron, WearableMaterial::Steel],
        }
    }
}

impl Generator<Wearable> for WearablePrototype {
    fn generate(&self) -> Wearable {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let material = if !self.possible_materials.is_empty() {
            let index = rng.gen_range(0..self.possible_materials.len());
            self.possible_materials.get(index).cloned()
        } else {
            None
        };

        let mut possible_descriptors: Vec<WearableDescriptor> =
            self.wearable_type.possible_descriptors().to_vec();
        let mut descriptors: Vec<WearableDescriptor> =
            self.wearable_type.necessary_descriptors().to_vec();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        Wearable {
            material,
            descriptors,
            defense: self.defense.clone(),
            wearable_type: self.wearable_type.clone(),
        }
    }
}
