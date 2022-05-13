#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fmt::Display;

use crate::utils::sentences::starters_and_joiners;

use super::items::character_item::{CharacterItem, CharacterItemView};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub equipment: Vec<CharacterItem>,
}

impl Inventory {
    pub fn count_weapons_at_ready(&self) -> usize {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.at_the_ready)
            .count()
    }

    pub fn find_item(&self, item_id: &Uuid) -> Option<CharacterItem> {
        self.equipment
            .iter()
            .find(|character_item| character_item.item.identifier.id.eq(item_id))
            .cloned()
    }

    pub fn add_item(&mut self, character_item: CharacterItem) {
        self.equipment.push(character_item)
    }

    pub fn remove_item(&mut self, item_id: &Uuid) -> Option<CharacterItem> {
        let index = self
            .equipment
            .iter()
            .enumerate()
            .find(|(_, character_item)| character_item.item.identifier.id.eq(item_id))
            .map(|(index, _)| index);

        match index {
            Some(it) => Some(self.equipment.remove(it)),
            None => None,
        }
    }

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

    pub fn readied_weapons(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.is_at_the_ready())
            .cloned()
            .collect()
    }

    pub fn non_readied_weapons(&self) -> Vec<&CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && !item.is_at_the_ready())
            .collect()
    }

    pub fn unequipped_weapons(&self) -> Vec<&CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && !item.is_equipped())
            .collect()
    }

    pub fn strongest_non_readied_weapon(&self) -> Option<&CharacterItem> {
        self.non_readied_weapons()
            .into_iter()
            .max_by(|a, b| a.item.num_attack_rolls().cmp(&b.item.num_attack_rolls()))
    }

    pub fn strongest_unequipped_weapon(&self) -> Option<&CharacterItem> {
        self.unequipped_weapons()
            .into_iter()
            .max_by(|a, b| a.item.num_attack_rolls().cmp(&b.item.num_attack_rolls()))
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct InventoryView {
    pub equipment: Vec<CharacterItemView>,
}

impl InventoryView {
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
        let equipped_weapons = self.equipped_weapons();

        self.item_description(equipped_weapons, "has no visible weapons.", starter)
    }

    pub fn wearables_description(&self, starter: &str) -> String {
        let equipped_wearables = self.equipped_wearables();
        self.item_description(equipped_wearables, "is wearing... nothing?", starter)
    }

    fn item_description(
        &self,
        items: Vec<CharacterItemView>,
        none_visible_text: &str,
        starter: &str,
    ) -> String {
        let visible_items: Vec<&CharacterItemView> = items
            .iter()
            .filter(|character_item| !character_item.is_hidden.unwrap_or(false))
            .collect();
        if visible_items.is_empty() {
            return format!("{} {}", starter, none_visible_text);
        }

        let intro_text = if starter == "It" {
            format!("{} is ", &starter)
        } else {
            format!("{} ", &starter)
        };

        let mut item_text: Vec<String> = vec![intro_text];

        for (index, character_item) in visible_items.iter().enumerate() {
            let (starters, joiners) = starters_and_joiners(&character_item.item);
            if index == 0 {
                item_text.push(format!(
                    "{} ",
                    starters.get_starter(character_item.is_multiple)
                ));
            }

            let description = character_item.item.describe();

            if index == visible_items.len() - 1 && visible_items.len() != 1 {
                item_text.push(", and ".to_string());
            } else if index > 0 {
                item_text.push(", ".to_string());
            }

            if index == 0 {
                item_text.push(description);
            } else {
                item_text.push(format!(
                    "{} {}",
                    joiners.get_joiner(character_item.is_multiple),
                    description
                ));
            }
        }

        item_text.push(".".to_string());
        item_text.join("")
    }

    pub fn equipped_wearables(&self) -> Vec<CharacterItemView> {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.is_equipped())
            .cloned()
            .collect()
    }

    pub fn equipped_weapons(&self) -> Vec<CharacterItemView> {
        self.equipment
            .iter()
            .filter(|item| item.is_weapon() && item.is_equipped())
            .cloned()
            .collect()
    }
}

impl Display for InventoryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe("It", "It"))
    }
}

#[cfg(test)]
mod inventory_tests {
    use crate::{
        components::{
            identifier::Identifier,
            items::{
                character_item::CharacterItem, descriptor::Descriptor, item::Item,
                item_type::ItemType, location_tag::LocationTag,
            },
            material::Material,
            tag::Tag,
        },
        systems::view::inventory::look_at,
    };

    use super::Inventory;

    #[test]
    fn display_with_multiple_weapons() {
        let long_sword = Item {
            identifier: Identifier::default(),
            attack: None,
            item_type: ItemType::LongSword,
            descriptors: vec![Descriptor::Broken],
            material: None,
            defense: None,
            tags: vec![Tag::Blade],
        };
        let short_sword = Item {
            identifier: Identifier::default(),
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
                    is_multiple: false,
                    at_the_ready: true,
                    equipped_location: LocationTag::Hand,
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: false,
                    is_multiple: false,
                    at_the_ready: true,
                    equipped_location: LocationTag::Hand,
                },
            ],
        };

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("a broken long sword"));
        assert!(description.contains(", and"));
        assert!(description.contains("rusty dull short sword."));
    }

    #[test]
    fn display_with_one_weapon() {
        let long_sword = Item {
            identifier: Identifier::default(),
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
                is_multiple: false,
                at_the_ready: true,
                equipped_location: LocationTag::Hand,
            }],
        };

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_hidden_weapon_does_not_describe_them() {
        let long_sword = Item {
            identifier: Identifier::default(),
            attack: None,
            item_type: ItemType::LongSword,
            descriptors: vec![Descriptor::Broken],
            material: None,
            tags: vec![Tag::Blade],
            defense: None,
        };
        let short_sword = Item {
            identifier: Identifier::default(),
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
                    is_multiple: false,
                    at_the_ready: true,
                    equipped_location: LocationTag::Hand,
                },
                CharacterItem {
                    item: short_sword,
                    is_hidden: true,
                    is_multiple: false,
                    at_the_ready: true,
                    equipped_location: LocationTag::Hand,
                },
            ],
        };

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("a broken long sword"));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_one_wearable() {
        let chain_mail = Item {
            identifier: Identifier::default(),
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
                is_multiple: false,
                at_the_ready: true,
                equipped_location: LocationTag::Body,
            }],
        };

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("set of drab steel plate helmet."));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_two_wearables() {
        let plate = Item {
            identifier: Identifier::default(),
            item_type: ItemType::Breastplate,
            material: Some(Material::Steel),
            descriptors: vec![Descriptor::Drab],
            defense: None,
            attack: None,
            tags: vec![Tag::Armour],
        };

        let shackles = Item {
            identifier: Identifier::default(),
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
                    item: plate,
                    is_hidden: false,
                    is_multiple: false,
                    at_the_ready: true,
                    equipped_location: LocationTag::Body,
                },
                CharacterItem {
                    item: shackles,
                    is_hidden: false,
                    is_multiple: true,
                    at_the_ready: true,
                    equipped_location: LocationTag::Wrist,
                },
            ],
        };

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("drab steel breast plate"));
        assert!(description.contains("rusty iron shackles."));
    }
}
