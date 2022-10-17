#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    items::{CharacterItem, CharacterItemView, Item},
    Attack, Defense,
};

#[derive(Clone, Debug, Default)]
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

    pub fn count_wearables_at_ready(&self) -> usize {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.at_the_ready)
            .count()
    }

    pub fn find_item(&self, item_id: &Uuid) -> Option<CharacterItem> {
        self.equipment
            .iter()
            .find(|character_item| character_item.item.id.eq(item_id))
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
            .find(|(_, character_item)| character_item.item.id.eq(item_id))
            .map(|(index, _)| index);

        match index {
            Some(it) => Some(self.equipment.remove(it)),
            None => None,
        }
    }

    pub fn equipped_wearables(&self) -> Vec<CharacterItem> {
        self.equipment
            .iter()
            .filter(|item| item.is_wearable() && item.is_at_the_ready())
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

    pub fn strongest_non_readied_weapon(&self) -> Option<&CharacterItem> {
        self.non_readied_weapons()
            .into_iter()
            .max_by(|a, b| a.item.num_attack_rolls().cmp(&b.item.num_attack_rolls()))
    }

    pub fn full_attack(&self) -> Option<Attack> {
        self.equipment
            .iter()
            .filter_map(|character_item| {
                if character_item.at_the_ready {
                    character_item.item.attack.clone()
                } else {
                    None
                }
            })
            .reduce(|accum, item| Attack {
                num_rolls: accum.num_rolls + item.num_rolls,
                modifier: accum.modifier + item.modifier,
                effects: accum
                    .effects
                    .into_iter()
                    .chain(item.effects.into_iter())
                    .collect(),
            })
    }

    pub fn full_defense(&self) -> Option<Defense> {
        self.equipment
            .iter()
            .filter_map(|character_item| {
                if character_item.at_the_ready {
                    character_item.item.defense.clone()
                } else {
                    None
                }
            })
            .reduce(|accum, item| Defense {
                damage_resistance: accum.damage_resistance + item.damage_resistance,
            })
    }

    pub fn drop_all(&mut self) -> Vec<Item> {
        let mut items: Vec<CharacterItem> = Vec::new();
        items.append(&mut self.equipment);
        items.into_iter().map(|ci| ci.item).collect()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Inventory"))]
pub struct InventoryView {
    pub equipment: Vec<CharacterItemView>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::components::{
        damage::AttackEffect,
        items::{CharacterItem, Item, ItemType, LocationTag},
        Attack, Defense,
    };

    use super::Inventory;

    #[test]
    fn drop_all() {
        let mut inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::Spear,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: Some(Attack {
                            num_rolls: 2,
                            modifier: 2,
                            effects: vec![AttackEffect::Crushing],
                        }),
                        defense: None,
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Hand,
                    at_the_ready: true,
                },
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::LongSword,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: Some(Attack {
                            num_rolls: 1,
                            modifier: -2,
                            effects: vec![AttackEffect::Sharp],
                        }),
                        defense: None,
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Hand,
                    at_the_ready: true,
                },
            ],
        };

        let items = inventory.drop_all();

        assert_eq!(0, inventory.equipment.len());
        assert_eq!(2, items.len());
    }

    #[test]
    fn full_attack() {
        let inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::Spear,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: Some(Attack {
                            num_rolls: 2,
                            modifier: 2,
                            effects: vec![AttackEffect::Crushing],
                        }),
                        defense: None,
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Hand,
                    at_the_ready: true,
                },
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::LongSword,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: Some(Attack {
                            num_rolls: 1,
                            modifier: -2,
                            effects: vec![AttackEffect::Sharp],
                        }),
                        defense: None,
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Hand,
                    at_the_ready: true,
                },
            ],
        };

        let merged = inventory.full_attack();
        assert!(merged.is_some());
        let attack = merged.unwrap();
        assert_eq!(attack.num_rolls, 3);
        assert_eq!(attack.modifier, 0);
        assert_eq!(
            attack.effects,
            vec![AttackEffect::Crushing, AttackEffect::Sharp]
        );
    }

    #[test]
    fn full_defense() {
        let inventory = Inventory {
            equipment: vec![
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::PlateBoots,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: None,
                        defense: Some(Defense {
                            damage_resistance: 2,
                        }),
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Feet,
                    at_the_ready: true,
                },
                CharacterItem {
                    item: Item {
                        id: Uuid::new_v4(),
                        name: None,
                        item_type: ItemType::PlateGauntlets,
                        tags: Vec::new(),
                        descriptors: Vec::new(),
                        material: None,
                        attack: None,
                        defense: Some(Defense {
                            damage_resistance: 6,
                        }),
                        consumable: None,
                        throwable: None,
                    },
                    equipped_location: LocationTag::Hand,
                    at_the_ready: true,
                },
            ],
        };

        let merged = inventory.full_defense();
        assert!(merged.is_some());
        let attack = merged.unwrap();
        assert_eq!(attack.damage_resistance, 8);
    }
}
