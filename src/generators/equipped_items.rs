use std::fmt::{Debug, Display};

use rand::Rng;

use crate::components::equipped_item::EquippedItem;

use super::generator::Generator;

pub struct EquippedItemPrototype<T: Display + Clone + Debug> {
    pub generator: Box<dyn Generator<T>>,
    pub hidden_chance: usize,
    pub multiple: bool,
    pub equipped_locations: Vec<String>,
    pub equipped_location_chance: usize,
}

impl<T: Display + Clone + Debug> Generator<EquippedItem<T>> for EquippedItemPrototype<T> {
    fn generate(&self) -> EquippedItem<T> {
        let item = self.generator.generate();

        let mut rng = rand::thread_rng();
        let hidden_roll: usize = rng.gen_range(0..=100);

        let equipped_location_roll: usize = rng.gen_range(0..=100);
        let equipped_location = if equipped_location_roll <= self.equipped_location_chance {
            let index = rng.gen_range(0..self.equipped_locations.len());
            match self.equipped_locations.get(index) {
                Some(equipped_location) => equipped_location.clone(),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        };

        EquippedItem {
            item,
            equipped_location,
            hidden: hidden_roll <= self.hidden_chance,
            multiple: self.multiple,
        }
    }
}
