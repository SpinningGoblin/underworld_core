#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

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

        for (index, item) in visible_items.iter().enumerate() {
            let (starters, joiners) = starters_and_joiners(&item.item);
            if index == 0 {
                item_text.push(format!("{} ", starters.get_starter(item.is_multiple)));
            }

            let description = match &item.location_descriptor {
                Some(location_descriptor) => {
                    format!("{} {}", item.item.describe(), location_descriptor)
                        .trim()
                        .to_string()
                }
                None => item.item.describe(),
            };

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
                    joiners.get_joiner(item.is_multiple),
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
                item_type::ItemType, location_descriptor::LocationDescriptor,
                location_tag::LocationTag,
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

        let description = look_at(&inventory, true, true, true).to_string();
        assert!(description.contains("a broken long sword"));
        assert!(description.contains(", and"));
        assert!(description.contains("rusty dull short sword sheathed at its hip."));
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
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
                equipped_location_tags: vec![LocationTag::Equipped],
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
                location_descriptor: LocationDescriptor::None,
                is_multiple: false,
                equipped_location_tags: vec![LocationTag::Equipped],
            }],
        };

        let view = look_at(&inventory, true, true, true).to_string();
        let description = view.to_string();
        assert!(description.contains("set of drab steel plate helmet."));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_two_wearables() {
        let chain_mail = Item {
            identifier: Identifier::default(),
            item_type: ItemType::PlateHelmet,
            material: Some(Material::Steel),
            descriptors: vec![Descriptor::SetOf, Descriptor::Drab],
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

        let view = look_at(&inventory, true, true, true).to_string();
        let description = view.to_string();
        assert!(description.contains("set of drab steel plate"));
        assert!(description.contains("rusty iron shackles dangling from its wrists."));
        assert!(description.contains(", and"));
    }
}
