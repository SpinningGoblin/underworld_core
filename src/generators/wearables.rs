use std::ops::Range;

use rand::Rng;

use crate::components::{
    defense::Defense,
    material::{BuiltWithMaterial, Material},
    object_descriptor::ObjectDescriptor,
    wearables::{wearable::Wearable, wearable_type::WearableType},
};

use super::generator::Generator;

struct WearablePrototype {
    pub wearable_type: WearableType,
    pub num_descriptors: Range<usize>,
    pub defense: Option<Defense>,
    pub possible_materials: Vec<Material>,
}

pub struct WearableGenerator;

impl WearableGenerator {
    pub fn for_wearable_type(wearable_type: &WearableType) -> impl Generator<Wearable> {
        match *wearable_type {
            WearableType::Breastplate => WearablePrototype::breastplate(),
            WearableType::Cloak => WearablePrototype::cloak(),
            WearableType::Shirt => WearablePrototype::shirt(),
            WearableType::PlateHelmet => WearablePrototype::plate_helmet(),
            WearableType::Shackles => WearablePrototype::shackles(),
            WearableType::Mask => WearablePrototype::mask(),
            WearableType::Trousers => WearablePrototype::trousers(),
            WearableType::Crown => WearablePrototype::crown(),
            WearableType::Boots => WearablePrototype::boots(),
            WearableType::Gloves => WearablePrototype::gloves(),
            WearableType::LoinCloth => WearablePrototype::loin_cloth(),
            WearableType::PlateBoots => WearablePrototype::plate_boots(),
            WearableType::PlateGauntlets => WearablePrototype::plate_gauntlets(),
            WearableType::Vest => WearablePrototype::vest(),
        }
    }
}

impl WearablePrototype {
    pub fn build(
        wearable_type: WearableType,
        num_descriptors: Range<usize>,
        defense: Option<Defense>,
    ) -> WearablePrototype {
        let possible_materials = wearable_type.possible_materials();
        Self {
            wearable_type,
            num_descriptors,
            defense,
            possible_materials,
        }
    }

    pub fn breastplate() -> Self {
        Self::build(
            WearableType::Breastplate,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn boots() -> Self {
        Self::build(
            WearableType::Boots,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn gloves() -> Self {
        Self::build(
            WearableType::Gloves,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn loin_cloth() -> Self {
        Self::build(
            WearableType::LoinCloth,
            0..3,
            Some(Defense {
                minimum: 0,
                maximum: 2,
            }),
        )
    }

    pub fn vest() -> Self {
        Self::build(
            WearableType::Vest,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn cloak() -> Self {
        Self::build(
            WearableType::Cloak,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn crown() -> Self {
        Self::build(
            WearableType::Crown,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn mask() -> Self {
        Self::build(
            WearableType::Mask,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn shirt() -> Self {
        Self::build(
            WearableType::Shirt,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn trousers() -> Self {
        Self::build(
            WearableType::Trousers,
            0..3,
            Some(Defense {
                minimum: 1,
                maximum: 3,
            }),
        )
    }

    pub fn plate_helmet() -> Self {
        Self::build(
            WearableType::PlateHelmet,
            0..3,
            Some(Defense {
                minimum: 3,
                maximum: 6,
            }),
        )
    }

    pub fn plate_boots() -> Self {
        Self::build(
            WearableType::PlateBoots,
            0..3,
            Some(Defense {
                minimum: 3,
                maximum: 6,
            }),
        )
    }

    pub fn plate_gauntlets() -> Self {
        Self::build(
            WearableType::PlateGauntlets,
            0..3,
            Some(Defense {
                minimum: 3,
                maximum: 6,
            }),
        )
    }

    pub fn shackles() -> Self {
        Self::build(WearableType::Shackles, 0..3, None)
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

        let mut possible_descriptors: Vec<ObjectDescriptor> = match &material {
            Some(material) => ObjectDescriptor::matches_two_tagged(&self.wearable_type, material),
            None => ObjectDescriptor::matches_tagged(&self.wearable_type),
        };
        let mut descriptors: Vec<ObjectDescriptor> =
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
