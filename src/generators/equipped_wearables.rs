use crate::components::wearable::Wearable;

use super::{equipped_items::EquippedItemPrototype, wearables::WearablePrototype};

impl EquippedItemPrototype<Wearable> {
    pub fn armour(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::armour()),
            equipped_locations: Vec::new(),
            multiple: false,
        }
    }

    pub fn cloak(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::cloak()),
            equipped_locations: vec!["hanging loosely".to_string()],
            multiple: false,
        }
    }

    pub fn clothing(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::clothing()),
            equipped_locations: vec!["".to_string()],
            multiple: false,
        }
    }

    pub fn plate_mail(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::plate_mail()),
            equipped_locations: vec!["".to_string()],
            multiple: false,
        }
    }

    pub fn shackles(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearablePrototype::shackles()),
            equipped_locations: vec!["".to_string()],
            multiple: true,
        }
    }
}
