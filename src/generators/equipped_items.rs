use std::fmt::{Debug, Display};

use rand::Rng;

use crate::components::equipped_item::{EquipLocationDescriptor, Equippable, EquippedItem};

use super::generator::Generator;

pub struct EquippedItemPrototype<T: Display + Clone + Debug + Equippable> {
    pub generator: Box<dyn Generator<T>>,
    pub hidden_chance: usize,
    pub multiple: bool,
    pub equipped_location_chance: usize,
}

impl<T: Display + Clone + Debug + Equippable> EquippedItemPrototype<T> {
    fn equipped_location(&self, item: &T) -> EquipLocationDescriptor {
        let mut rng = rand::thread_rng();

        if item.possible_equip_locations().is_empty() {
            return EquipLocationDescriptor::None;
        }

        let equipped_location_roll: usize = rng.gen_range(0..=100);
        if equipped_location_roll > self.equipped_location_chance {
            return EquipLocationDescriptor::None;
        }

        let index = rng.gen_range(0..item.possible_equip_locations().len());
        item.possible_equip_locations()
            .get(index)
            .cloned()
            .unwrap_or_default()
    }
}

impl<T: Display + Clone + Debug + Equippable> Generator<EquippedItem<T>>
    for EquippedItemPrototype<T>
{
    fn generate(&self) -> EquippedItem<T> {
        let item = self.generator.generate();

        let mut rng = rand::thread_rng();
        let hidden_roll: usize = rng.gen_range(0..=100);
        let equipped_location = self.equipped_location(&item);

        EquippedItem {
            item,
            equipped_location,
            hidden: hidden_roll <= self.hidden_chance,
            multiple: self.multiple,
        }
    }
}
