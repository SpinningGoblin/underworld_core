use std::ops::Range;

use rand::Rng;

use crate::components::{
    defense::Defense,
    wearable::{Wearable, WearableDescriptor, WearableMaterial, WearableType},
};

use super::generator::Generator;

pub struct WearablePrototype {
    pub wearable_type: WearableType,
    pub descriptors: Vec<WearableDescriptor>,
    pub possible_descriptors: Vec<WearableDescriptor>,
    pub num_descriptors: Range<usize>,
    pub defense: Option<Defense>,
    pub possible_materials: Vec<WearableMaterial>,
}

impl WearablePrototype {
    pub fn all() -> Vec<Box<dyn Generator<Wearable>>> {
        return vec![
            Box::new(Self::armour()),
            Box::new(Self::cloak()),
            Box::new(Self::clothing()),
            Box::new(Self::plate_mail()),
            Box::new(Self::shackles()),
        ];
    }

    pub fn armour() -> Self {
        Self {
            wearable_type: WearableType::Armour,
            descriptors: Vec::new(),
            possible_descriptors: vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Rusty,
                WearableDescriptor::Stained,
            ],
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
            descriptors: Vec::new(),
            num_descriptors: 0..3,
            possible_descriptors: vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
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
            descriptors: Vec::new(),
            num_descriptors: 0..3,
            possible_descriptors: vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Colourful,
                WearableDescriptor::Dingy,
                WearableDescriptor::Drab,
                WearableDescriptor::IllFitting,
                WearableDescriptor::Shimmering,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
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
            descriptors: Vec::new(),
            possible_descriptors: vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
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
            descriptors: vec![WearableDescriptor::SetOf],
            possible_descriptors: vec![
                WearableDescriptor::Bloodstained,
                WearableDescriptor::Rusty,
                WearableDescriptor::Shiny,
                WearableDescriptor::Stained,
            ],
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

        let mut possible_descriptors: Vec<WearableDescriptor> = self.possible_descriptors.to_vec();
        let mut descriptors: Vec<WearableDescriptor> = self.descriptors.to_vec();
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
