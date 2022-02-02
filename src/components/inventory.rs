#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use super::{equipped::Equipped, weapons::weapon::Weapon, wearables::wearable::Wearable};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub equipped: Equipped,
    pub carried_weapons: Vec<Weapon>,
    pub carried_wearables: Vec<Wearable>,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe("It", "It"))
    }
}

impl Inventory {
    pub fn describe(&self, weapon_starter: &str, wearable_starter: &str) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        let weapon_description = self.weapon_description(weapon_starter);
        if !weapon_description.is_empty() {
            descriptions.push(weapon_description);
        }

        let wearable_description = self.wearables_description(wearable_starter);
        if !wearable_description.is_empty() {
            descriptions.push(wearable_description);
        }

        descriptions.join(" ")
    }

    fn weapon_description(&self, starter: &str) -> String {
        self.equipped.weapon_description(starter)
    }

    fn wearables_description(&self, starter: &str) -> String {
        self.equipped.wearables_description(starter)
    }
}

#[cfg(test)]
mod inventory_tests {
    use crate::components::{
        equipped::{
            equip_location_descriptor::EquipLocationDescriptor, equipped_item::EquippedItem,
            Equipped,
        },
        item_descriptor::ItemDescriptor,
        material::Material,
        weapons::{weapon::Weapon, weapon_type::WeaponType},
        wearables::{wearable::Wearable, wearable_type::WearableType},
    };

    use super::Inventory;

    #[test]
    fn display_with_multiple_weapons() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Broken],
            material: None,
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![ItemDescriptor::Rusty, ItemDescriptor::Dull],
            material: None,
        };
        let inventory = Inventory {
            equipped: Equipped {
                weapons: vec![
                    EquippedItem {
                        item: long_sword,
                        hidden: false,
                        equipped_location: EquipLocationDescriptor::None,
                        multiple: false,
                    },
                    EquippedItem {
                        item: short_sword,
                        hidden: false,
                        equipped_location: EquipLocationDescriptor::SheathedAtHip,
                        multiple: false,
                    },
                ],
                wearables: Vec::new(),
            },
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(description.contains(", and"));
        assert!(description.contains("rusty dull short sword sheathed at its hip."));
    }

    #[test]
    fn display_with_one_weapon() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Broken],
            material: None,
        };
        let inventory = Inventory {
            equipped: Equipped {
                weapons: vec![EquippedItem {
                    item: long_sword,
                    hidden: false,
                    equipped_location: EquipLocationDescriptor::None,
                    multiple: false,
                }],
                wearables: Vec::new(),
            },
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_hidden_weapon_does_not_describe_them() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ItemDescriptor::Broken],
            material: None,
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![ItemDescriptor::Rusty, ItemDescriptor::Dull],
            material: None,
        };
        let inventory = Inventory {
            equipped: Equipped {
                weapons: vec![
                    EquippedItem {
                        item: long_sword,
                        hidden: false,
                        equipped_location: EquipLocationDescriptor::None,
                        multiple: false,
                    },
                    EquippedItem {
                        item: short_sword,
                        hidden: true,
                        equipped_location: EquipLocationDescriptor::StrappedToThigh,
                        multiple: false,
                    },
                ],
                wearables: Vec::new(),
            },
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_one_wearable() {
        let chain_mail = Wearable {
            wearable_type: WearableType::PlateHelmet,
            material: Some(Material::Steel),
            descriptors: vec![ItemDescriptor::SetOf, ItemDescriptor::Drab],
            defense: None,
        };

        let inventory = Inventory {
            equipped: Equipped {
                weapons: Vec::new(),
                wearables: vec![EquippedItem {
                    item: chain_mail,
                    hidden: false,
                    equipped_location: EquipLocationDescriptor::None,
                    multiple: false,
                }],
            },
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate helmet."));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_two_wearables() {
        let chain_mail = Wearable {
            wearable_type: WearableType::PlateHelmet,
            material: Some(Material::Steel),
            descriptors: vec![ItemDescriptor::SetOf, ItemDescriptor::Drab],
            defense: None,
        };

        let shackles = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(Material::Iron),
            descriptors: vec![ItemDescriptor::Rusty],
            defense: None,
        };

        let inventory = Inventory {
            equipped: Equipped {
                weapons: Vec::new(),
                wearables: vec![
                    EquippedItem {
                        item: chain_mail,
                        hidden: false,
                        equipped_location: EquipLocationDescriptor::None,
                        multiple: false,
                    },
                    EquippedItem {
                        item: shackles,
                        hidden: false,
                        equipped_location: EquipLocationDescriptor::DanglingFromWrists,
                        multiple: true,
                    },
                ],
            },
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate"));
        assert!(description.contains("rusty iron shackles dangling from its wrists."));
        assert!(description.contains(", and"));
    }
}
