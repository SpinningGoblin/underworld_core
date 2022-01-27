#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use rand::{thread_rng, Rng};

use super::{
    equipped_item::EquippedItem,
    weapon::Weapon,
    wearable::{Wearable, WearableType},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Inventory {
    pub equipped_weapons: Vec<EquippedItem<Weapon>>,
    pub equipped_wearables: Vec<EquippedItem<Wearable>>,
    pub carried_weapons: Vec<Weapon>,
    pub carried_wearables: Vec<Wearable>,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();

        let weapon_description = self.weapon_description();
        if !weapon_description.is_empty() {
            descriptions.push(weapon_description);
        }

        let wearable_description = self.wearables_description();
        if !wearable_description.is_empty() {
            descriptions.push(wearable_description);
        }

        write!(f, "{}", descriptions.join(" "))
    }
}

impl Inventory {
    fn weapon_description(&self) -> String {
        let sentence_starters = SentenceStarters::default();
        let sentence_joiners = SentenceJoiners::default();
        let mut weapons: Vec<String> = vec!["It ".to_string()];

        self.equipped_weapons
            .iter()
            .filter(|equipped_weapon| !equipped_weapon.hidden)
            .enumerate()
            .for_each(|(index, equipped_weapon)| {
                if index == 0 {
                    weapons.push(format!(
                        "{} ",
                        sentence_starters.get_weapon_starter(equipped_weapon.multiple)
                    ));
                }

                let weapon_description = format!(
                    "{} {}",
                    equipped_weapon.item, equipped_weapon.equipped_location
                )
                .trim_end()
                .to_string();

                if index == self.equipped_weapons.len() - 1 && self.equipped_weapons.len() != 1 {
                    weapons.push(", and ".to_string());
                } else if index > 0 {
                    weapons.push(", ".to_string());
                }

                if index == 0 {
                    weapons.push(weapon_description);
                } else {
                    weapons.push(format!(
                        "{} {}",
                        sentence_joiners.get_weapon_joiner(equipped_weapon.multiple),
                        weapon_description
                    ));
                }

                if index == self.equipped_weapons.len() - 1 || self.equipped_weapons.len() == 1 {
                    weapons.push(".".to_string());
                }
            });

        weapons.join("")
    }

    fn wearables_description(&self) -> String {
        let sentence_starters = SentenceStarters::default();
        let sentence_joiners = SentenceJoiners::default();
        let mut wearables: Vec<String> = vec!["It is ".to_string()];

        self.equipped_wearables
            .iter()
            .filter(|equipped_wearable| !equipped_wearable.hidden)
            .enumerate()
            .for_each(|(index, equipped_wearable)| {
                if index == 0 {
                    wearables.push(format!(
                        "{} ",
                        sentence_starters.get_wearable_starter(equipped_wearable.multiple)
                    ));
                }

                let wearable_description = format!(
                    "{} {}",
                    equipped_wearable.item, equipped_wearable.equipped_location
                )
                .trim_end()
                .to_string();

                if index == self.equipped_wearables.len() - 1 && self.equipped_wearables.len() != 1
                {
                    wearables.push(", and ".to_string());
                } else if index > 0 {
                    wearables.push(", ".to_string());
                }

                if index == 0 {
                    wearables.push(wearable_description);
                } else {
                    wearables.push(format!(
                        "{} {}",
                        sentence_joiners.get_wearable_joiner(equipped_wearable.multiple),
                        wearable_description
                    ));
                }

                if index == self.equipped_wearables.len() - 1 || self.equipped_wearables.len() == 1
                {
                    wearables.push(".".to_string());
                }
            });

        if self
            .equipped_wearables
            .iter()
            .filter(|e| e.item.wearable_type == WearableType::Cloak)
            .count()
            > 1
        {
            wearables.push(" Two cloaks...?".to_string());
        }

        wearables.join("")
    }
}

struct SentenceStarters {
    weapon_singular_starters: Vec<String>,
    weapon_plural_starters: Vec<String>,
    wearable_singular_starters: Vec<String>,
    wearable_plural_starters: Vec<String>,
}

impl Default for SentenceStarters {
    fn default() -> Self {
        Self {
            weapon_singular_starters: vec!["has a".to_string(), "carries a".to_string()],
            weapon_plural_starters: vec!["has some".to_string(), "carries some".to_string()],
            wearable_singular_starters: vec!["wearing a".to_string()],
            wearable_plural_starters: vec!["wearing".to_string()],
        }
    }
}

impl SentenceStarters {
    fn get_weapon_starter(&self, plural: bool) -> &String {
        let mut rng = thread_rng();
        if plural {
            let i = rng.gen_range(0..self.weapon_plural_starters.len());
            self.weapon_plural_starters.get(i).unwrap()
        } else {
            let i = rng.gen_range(0..self.weapon_singular_starters.len());
            self.weapon_singular_starters.get(i).unwrap()
        }
    }

    fn get_wearable_starter(&self, plural: bool) -> &String {
        let mut rng = thread_rng();
        if plural {
            let i = rng.gen_range(0..self.wearable_plural_starters.len());
            self.wearable_plural_starters.get(i).unwrap()
        } else {
            let i = rng.gen_range(0..self.wearable_singular_starters.len());
            self.wearable_singular_starters.get(i).unwrap()
        }
    }
}

struct SentenceJoiners {
    weapon_singular_joiners: Vec<String>,
    weapon_plural_joiners: Vec<String>,
    wearable_singular_joiners: Vec<String>,
    wearable_plural_joiners: Vec<String>,
}

impl Default for SentenceJoiners {
    fn default() -> Self {
        Self {
            weapon_singular_joiners: vec!["a".to_string(), "one".to_string()],
            weapon_plural_joiners: vec!["some".to_string()],
            wearable_singular_joiners: vec!["a".to_string()],
            wearable_plural_joiners: vec!["some".to_string()],
        }
    }
}

impl SentenceJoiners {
    fn get_weapon_joiner(&self, plural: bool) -> &String {
        let mut rng = thread_rng();
        if plural {
            let i = rng.gen_range(0..self.weapon_plural_joiners.len());
            self.weapon_plural_joiners.get(i).unwrap()
        } else {
            let i = rng.gen_range(0..self.weapon_singular_joiners.len());
            self.weapon_singular_joiners.get(i).unwrap()
        }
    }

    fn get_wearable_joiner(&self, plural: bool) -> &String {
        let mut rng = thread_rng();
        if plural {
            let i = rng.gen_range(0..self.wearable_plural_joiners.len());
            self.wearable_plural_joiners.get(i).unwrap()
        } else {
            let i = rng.gen_range(0..self.wearable_singular_joiners.len());
            self.wearable_singular_joiners.get(i).unwrap()
        }
    }
}

#[cfg(test)]
mod inventory_tests {
    use crate::components::{
        equipped_item::{EquippedItem, EquippedLocation},
        weapon::{Weapon, WeaponDescriptor, WeaponType},
        wearable::{Wearable, WearableDescriptor, WearableMaterial, WearableType},
    };

    use super::Inventory;

    #[test]
    fn display_with_multiple_weapons() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![WeaponDescriptor::Broken],
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![WeaponDescriptor::Rusty, WeaponDescriptor::Dull],
        };
        let inventory = Inventory {
            equipped_weapons: vec![
                EquippedItem {
                    item: long_sword,
                    hidden: false,
                    equipped_location: EquippedLocation::None,
                    multiple: false,
                },
                EquippedItem {
                    item: short_sword,
                    hidden: false,
                    equipped_location: EquippedLocation::SheathedAtHip,
                    multiple: false,
                },
            ],
            equipped_wearables: Vec::new(),
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
            descriptors: vec![WeaponDescriptor::Broken],
        };
        let inventory = Inventory {
            equipped_weapons: vec![EquippedItem {
                item: long_sword,
                hidden: false,
                equipped_location: EquippedLocation::None,
                multiple: false,
            }],
            equipped_wearables: Vec::new(),
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
            descriptors: vec![WeaponDescriptor::Broken],
        };
        let short_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::ShortSword,
            descriptors: vec![WeaponDescriptor::Rusty, WeaponDescriptor::Dull],
        };
        let inventory = Inventory {
            equipped_weapons: vec![
                EquippedItem {
                    item: long_sword,
                    hidden: false,
                    equipped_location: EquippedLocation::None,
                    multiple: false,
                },
                EquippedItem {
                    item: short_sword,
                    hidden: true,
                    equipped_location: EquippedLocation::StrappedToThigh,
                    multiple: false,
                },
            ],
            equipped_wearables: Vec::new(),
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
            material: Some(WearableMaterial::Steel),
            descriptors: vec![WearableDescriptor::SetOf, WearableDescriptor::Drab],
            defense: None,
        };

        let inventory = Inventory {
            equipped_weapons: Vec::new(),
            equipped_wearables: vec![EquippedItem {
                item: chain_mail,
                hidden: false,
                equipped_location: EquippedLocation::None,
                multiple: false,
            }],
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
            material: Some(WearableMaterial::Steel),
            descriptors: vec![WearableDescriptor::SetOf, WearableDescriptor::Drab],
            defense: None,
        };

        let shackles = Wearable {
            wearable_type: WearableType::Shackles,
            material: Some(WearableMaterial::Iron),
            descriptors: vec![WearableDescriptor::Rusty],
            defense: None,
        };

        let inventory = Inventory {
            equipped_weapons: Vec::new(),
            equipped_wearables: vec![
                EquippedItem {
                    item: chain_mail,
                    hidden: false,
                    equipped_location: EquippedLocation::None,
                    multiple: false,
                },
                EquippedItem {
                    item: shackles,
                    hidden: false,
                    equipped_location: EquippedLocation::DanglingFromWrists,
                    multiple: true,
                },
            ],
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate"));
        assert!(description.contains("rusty iron shackles dangling from its wrists."));
        assert!(description.contains(", and"));
    }
}
