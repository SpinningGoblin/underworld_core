use crate::components::wearable::{Wearable, WearableType};

use super::{equipped_items::EquippedItemPrototype, wearables::WearableGenerator};

impl EquippedItemPrototype<Wearable> {
    pub fn breastplate(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::Breastplate,
            )),
            multiple: true,
        }
    }

    pub fn cloak(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(&WearableType::Cloak)),
            multiple: false,
        }
    }

    pub fn shirt(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(&WearableType::Shirt)),
            multiple: true,
        }
    }

    pub fn plate_mail(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::PlateHelmet,
            )),
            multiple: true,
        }
    }

    pub fn shackles(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::Shackles,
            )),
            multiple: false,
        }
    }
}
