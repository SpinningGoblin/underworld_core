#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::utils::sentences::{SentenceJoiners, SentenceStarters};

use super::items::character_item::CharacterItem;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub equipment: Vec<CharacterItem>,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe("It", "It"))
    }
}

impl Inventory {
    pub fn equipped_wearables(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.is_equipped())
            .cloned()
            .collect()
    }

    pub fn equipped_weapons(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.is_equipped())
            .cloned()
            .collect()
    }

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
        let inventory_weapons = self.equipped_weapons();

        let visible_weapons: Vec<&CharacterItem> = inventory_weapons
            .iter()
            .filter(|weapon| !weapon.is_hidden)
            .collect();
        if visible_weapons.is_empty() {
            return format!("{} has no visible weapons.", starter);
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
                weapon.item.describe(true),
                weapon.location_descriptor
            )
            .trim_end()
            .to_string();

            if index == inventory_weapons.len() - 1 && inventory_weapons.len() != 1 {
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
        let inventory_wearables = self.equipped_wearables();
        let visible_wearables: Vec<&CharacterItem> = inventory_wearables
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
                wearable.item.describe(true),
                wearable.location_descriptor
            )
            .trim_end()
            .to_string();

            if index == inventory_wearables.len() - 1 && inventory_wearables.len() != 1 {
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
        items::{
            character_item::CharacterItem, descriptor::Descriptor, item::Item, item_type::ItemType,
            location_descriptor::LocationDescriptor, location_tag::LocationTag,
        },
        material::Material,
        tag::Tag,
    };

    use super::Inventory;

    #[test]
    fn display_with_multiple_weapons() {
        let long_sword = Item {
            attack: None,
            item_type: ItemType::LongSword,
            descriptors: vec![Descriptor::Broken],
            material: None,
            defense: None,
            tags: vec![Tag::Blade],
        };
        let short_sword = Item {
            attack: None,
            item_type: ItemType::ShortSword,
            descriptors: vec![Descriptor::Rusty, Descriptor::Dull],
            material: None,
            tags: vec![Tag::Blade],
            defense: None,
        };
        let inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: long_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::SheathedAtHip,
                    is_multiple: false,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
            ],
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(description.contains(", and"));
        assert!(description.contains("rusty dull short sword sheathed at its hip."));
    }

    #[test]
    fn display_with_one_weapon() {
        let long_sword = Item {
            attack: None,
            item_type: ItemType::LongSword,
            descriptors: vec![Descriptor::Broken],
            material: None,
            tags: vec![Tag::Blade],
            defense: None,
        };
        let inventory = Inventory {
            equipment: vec![CharacterItem {
                item: long_sword,
                is_hidden: false,
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
                equipped_location_tags: vec![LocationTag::Equipped],
            }],
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_hidden_weapon_does_not_describe_them() {
        let long_sword = Item {
            attack: None,
            item_type: ItemType::LongSword,
            descriptors: vec![Descriptor::Broken],
            material: None,
            tags: vec![Tag::Blade],
            defense: None,
        };
        let short_sword = Item {
            attack: None,
            item_type: ItemType::ShortSword,
            descriptors: vec![Descriptor::Rusty, Descriptor::Dull],
            material: None,
            tags: vec![Tag::Blade],
            defense: None,
        };
        let inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: long_sword,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: true,
                    location_descriptor: LocationDescriptor::StrappedToThigh,
                    is_multiple: false,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
            ],
        };

        let description = inventory.to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_one_wearable() {
        let chain_mail = Item {
            item_type: ItemType::PlateHelmet,
            material: Some(Material::Steel),
            descriptors: vec![Descriptor::SetOf, Descriptor::Drab],
            defense: None,
            attack: None,
            tags: vec![Tag::Armour],
        };

        let inventory = Inventory {
            equipment: vec![CharacterItem {
                item: chain_mail,
                is_hidden: false,
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
                equipped_location_tags: vec![LocationTag::Equipped],
            }],
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate helmet."));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_two_wearables() {
        let chain_mail = Item {
            item_type: ItemType::PlateHelmet,
            material: Some(Material::Steel),
            descriptors: vec![Descriptor::SetOf, Descriptor::Drab],
            defense: None,
            attack: None,
            tags: vec![Tag::Armour],
        };

        let shackles = Item {
            item_type: ItemType::Shackles,
            material: Some(Material::Iron),
            descriptors: vec![Descriptor::Rusty],
            defense: None,
            attack: None,
            tags: vec![Tag::Accessory],
        };

        let inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: chain_mail,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::None,
                    is_multiple: false,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
                CharacterItem {
                    item: shackles,
                    is_hidden: false,
                    location_descriptor: LocationDescriptor::DanglingFromWrists,
                    is_multiple: true,
                    equipped_location_tags: vec![LocationTag::Equipped],
                },
            ],
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate"));
        assert!(description.contains("rusty iron shackles dangling from its wrists."));
        assert!(description.contains(", and"));
    }
}
