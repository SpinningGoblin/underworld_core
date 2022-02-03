#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::utils::sentences::{SentenceJoiners, SentenceStarters};

use super::{
    character_item::CharacterItem, object::Object, weapons::weapon::Weapon,
    wearables::wearable::Wearable,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub weapons: Vec<CharacterItem<Weapon>>,
    pub wearables: Vec<CharacterItem<Wearable>>,
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

    pub fn weapon_description(&self, starter: &str) -> String {
        let visible_weapons: Vec<&CharacterItem<Weapon>> = self
            .weapons
            .iter()
            .filter(|weapon| !weapon.is_hidden)
            .collect();
        if visible_weapons.is_empty() {
            return format!("{} has no visible weapons", starter);
        }

        let starters = SentenceStarters::weapon_starters();
        let joiners = SentenceJoiners::weapon_joiners();
        let mut weapons: Vec<String> = vec![format!("{} ", starter)];

        for (index, weapon) in visible_weapons.iter().enumerate() {
            if index == 0 {
                weapons.push(format!("{} ", starters.get_starter(weapon.is_multiple)));
            }

            let description = format!(
                "{} {}",
                weapon.item.look_at(true),
                weapon.location_descriptor
            )
            .trim_end()
            .to_string();

            if index == self.weapons.len() - 1 && self.weapons.len() != 1 {
                weapons.push(", and ".to_string());
            } else if index > 0 {
                weapons.push(", ".to_string());
            }

            if index == 0 {
                weapons.push(description);
            } else {
                weapons.push(format!(
                    "{} {}",
                    joiners.get_joiner(weapon.is_multiple),
                    description
                ));
            }
        }

        weapons.push(".".to_string());
        weapons.join("")
    }

    pub fn wearables_description(&self, starter: &str) -> String {
        let visible_wearables: Vec<&CharacterItem<Wearable>> = self
            .wearables
            .iter()
            .filter(|equipped_wearable| !equipped_wearable.is_hidden)
            .collect();

        if visible_wearables.is_empty() {
            return format!("{} is wearing... nothing?", starter);
        }

        let starters = SentenceStarters::wearable_starters();
        let joiners = SentenceJoiners::wearable_joiners();
        let mut wearables: Vec<String> = vec![format!("{} is ", starter)];

        for (index, wearable) in visible_wearables.iter().enumerate() {
            if index == 0 {
                wearables.push(format!("{} ", starters.get_starter(wearable.is_multiple)));
            }

            let description = format!(
                "{} {}",
                wearable.item.look_at(true),
                wearable.location_descriptor
            )
            .trim_end()
            .to_string();

            if index == self.wearables.len() - 1 && self.wearables.len() != 1 {
                wearables.push(", and ".to_string());
            } else if index > 0 {
                wearables.push(", ".to_string());
            }

            if index == 0 {
                wearables.push(description);
            } else {
                wearables.push(format!(
                    "{} {}",
                    joiners.get_joiner(wearable.is_multiple),
                    description
                ));
            }
        }

        wearables.push(".".to_string());

        wearables.join("")
    }
}

#[cfg(test)]
mod inventory_tests {
    use crate::components::{
        character_item::CharacterItem,
        equipment::location_descriptor::LocationDescriptor,
        material::Material,
        object_descriptor::ObjectDescriptor,
        weapons::{weapon::Weapon, weapon_type::WeaponType},
        wearables::{wearable::Wearable, wearable_type::WearableType},
    };

    use super::Inventory;

    #[test]
    fn display_with_multiple_weapons() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![ObjectDescriptor::Broken],
            material: None,
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![ObjectDescriptor::Rusty, ObjectDescriptor::Dull],
            material: None,
        };
        let inventory = Inventory {
            weapons: vec![
                CharacterItem {
                    item: long_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::SheathedAtHip,
                    is_multiple: false,
                },
            ],
            wearables: Vec::new(),
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
            descriptors: vec![ObjectDescriptor::Broken],
            material: None,
        };
        let inventory = Inventory {
            weapons: vec![CharacterItem {
                item: long_sword,
                is_hidden: false,
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
            }],
            wearables: Vec::new(),
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
            descriptors: vec![ObjectDescriptor::Broken],
            material: None,
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![ObjectDescriptor::Rusty, ObjectDescriptor::Dull],
            material: None,
        };
        let inventory = Inventory {
            weapons: vec![
                CharacterItem {
                    item: long_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: true,
                    location_descriptor: LocationDescriptor::StrappedToThigh,
                    is_multiple: false,
                },
            ],
            wearables: Vec::new(),
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
            descriptors: vec![ObjectDescriptor::SetOf, ObjectDescriptor::Drab],
            defense: None,
        };

        let inventory = Inventory {
            weapons: Vec::new(),
            wearables: vec![CharacterItem {
                item: chain_mail,
                is_hidden: false,
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
            }],
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
            descriptors: vec![ObjectDescriptor::SetOf, ObjectDescriptor::Drab],
            defense: None,
        };

        let shackles = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(Material::Iron),
            descriptors: vec![ObjectDescriptor::Rusty],
            defense: None,
        };

        let inventory = Inventory {
            weapons: Vec::new(),
            wearables: vec![
                CharacterItem {
                    item: chain_mail,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                },
                CharacterItem {
                    item: shackles,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::DanglingFromWrists,
                    is_multiple: true,
                },
            ],
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate"));
        assert!(description.contains("rusty iron shackles dangling from its wrists."));
        assert!(description.contains(", and"));
    }
}
