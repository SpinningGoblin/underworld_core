#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

use crate::components::item::{EquippableItem, Item};

use super::equip_location_descriptor::EquipLocationDescriptor;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct EquippedItem<T: Display + Clone + Debug + Item + EquippableItem> {
    pub item: T,
    pub hidden: bool,
    pub equipped_location: EquipLocationDescriptor,
    pub multiple: bool,
}

#[cfg(test)]
#[cfg(feature = "serialization")]
#[cfg(feature = "json")]
mod serialization_tests {
    use crate::components::{
        equipped::equip_location_descriptor::EquipLocationDescriptor,
        item_descriptor::ItemDescriptor,
        weapons::{weapon::Weapon, weapon_type::WeaponType},
    };

    use super::EquippedItem;

    #[test]
    fn serialize() {
        let weapon = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Dull],
            material: None,
        };
        let equipped_item = EquippedItem {
            item: weapon,
            hidden: false,
            equipped_location: EquipLocationDescriptor::StrappedToThigh,
            multiple: false,
        };
        let serialized = serde_json::to_string(&equipped_item).unwrap();
        let deserialized: EquippedItem<Weapon> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(WeaponType::LongSword, deserialized.item.weapon_type);
    }
}
