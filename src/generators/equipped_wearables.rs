use crate::components::wearable::Wearable;

use super::{equipped_items::EquippedItemPrototype, wearables::WearablePrototype};

impl EquippedItemPrototype<Wearable> {
    pub fn armour(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::armour()),
            multiple: true,
        }
    }

    pub fn cloak(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::cloak()),
            multiple: false,
        }
    }

    pub fn clothing(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::clothing()),
            multiple: true,
        }
    }

    pub fn plate_mail(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::plate_mail()),
            multiple: true,
        }
    }

    pub fn shackles(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::shackles()),
            multiple: false,
        }
    }
}
