#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct EquippedItem<T: Display + Clone + Debug> {
    pub item: T,
    pub hidden: bool,
    pub equipped_location: String,
    pub multiple: bool,
}

#[cfg(test)]
#[cfg(feature = "serialization")]
#[cfg(feature = "json")]
mod serialization_tests {
    use crate::components::weapon::{Weapon, WeaponDescriptor, WeaponType};

    use super::EquippedItem;

    #[test]
    fn serialize() {
        let weapon = Weapon {
            attack: None,
            weapon_type: crate::components::weapon::WeaponType::LongSword,
            descriptors: vec![WeaponDescriptor::Dull],
        };
        let equipped_item = EquippedItem {
            item: weapon,
            hidden: false,
            equipped_location: "somewhere".to_string(),
            multiple: false,
        };
        let serialized = serde_json::to_string(&equipped_item).unwrap();
        let deserialized: EquippedItem<Weapon> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(WeaponType::LongSword, deserialized.item.weapon_type);
    }
}
