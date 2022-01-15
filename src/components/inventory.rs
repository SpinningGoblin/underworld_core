use std::fmt::Display;

use rand::{thread_rng, Rng};

use super::{
    weapon::{EquippedWeapon, Weapon},
    wearable::{EquippedWearable, Wearable},
};

#[derive(Clone, Debug)]
pub struct Inventory {
    pub equipped_weapons: Vec<EquippedWeapon>,
    pub equipped_wearables: Vec<EquippedWearable>,
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
        let mut weapons: Vec<String> = Vec::new();

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

                let weapon_description = if equipped_weapon.equipped_location.is_empty() {
                    format!("{}", equipped_weapon.weapon)
                } else {
                    format!(
                        "{} {}",
                        equipped_weapon.weapon, equipped_weapon.equipped_location
                    )
                };

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
                        sentence_joiners.get_joiner(equipped_weapon.multiple),
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
        let mut wearables: Vec<String> = Vec::new();

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

                let wearable_description = if equipped_wearable.equipped_location.is_empty() {
                    format!("{}", equipped_wearable.wearable)
                } else {
                    format!(
                        "{} {}",
                        equipped_wearable.wearable, equipped_wearable.equipped_location
                    )
                };

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
                        sentence_joiners.get_joiner(equipped_wearable.multiple),
                        wearable_description
                    ));
                }

                if index == self.equipped_wearables.len() - 1 || self.equipped_wearables.len() == 1
                {
                    wearables.push(".".to_string());
                }
            });

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
            weapon_singular_starters: vec!["Has a".to_string(), "Carries a".to_string()],
            weapon_plural_starters: vec!["Has some".to_string(), "Carries some".to_string()],
            wearable_singular_starters: vec!["Wearing a".to_string()],
            wearable_plural_starters: vec!["Wearing some".to_string()],
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
    singular_joiners: Vec<String>,
    plural_joiners: Vec<String>,
}

impl Default for SentenceJoiners {
    fn default() -> Self {
        Self {
            singular_joiners: vec!["a".to_string(), "one".to_string()],
            plural_joiners: vec!["some".to_string(), "multiple".to_string()],
        }
    }
}

impl SentenceJoiners {
    fn get_joiner(&self, plural: bool) -> &String {
        let mut rng = thread_rng();
        if plural {
            let i = rng.gen_range(0..self.plural_joiners.len());
            self.plural_joiners.get(i).unwrap()
        } else {
            let i = rng.gen_range(0..self.singular_joiners.len());
            self.singular_joiners.get(i).unwrap()
        }
    }
}

#[cfg(test)]
mod inventory_tests {
    use crate::components::{
        weapon::{EquippedWeapon, Weapon, WeaponDescriptor, WeaponType},
        wearable::{
            EquippedWearable, Wearable, WearableDescriptor, WearableMaterial, WearableType,
        },
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
                EquippedWeapon {
                    weapon: long_sword,
                    hidden: false,
                    equipped_location: "".to_string(),
                    multiple: false,
                },
                EquippedWeapon {
                    weapon: short_sword,
                    hidden: false,
                    equipped_location: "in a sheath hanging from their hip".to_string(),
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
        assert!(description.contains("rusty dull short sword in a sheath hanging from their hip."));
    }

    #[test]
    fn display_with_one_weapon() {
        let long_sword = Weapon {
            attack: None,
            weapon_type: WeaponType::LongSword,
            descriptors: vec![WeaponDescriptor::Broken],
        };
        let inventory = Inventory {
            equipped_weapons: vec![EquippedWeapon {
                weapon: long_sword,
                hidden: false,
                equipped_location: "".to_string(),
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
                EquippedWeapon {
                    weapon: long_sword,
                    hidden: false,
                    equipped_location: "".to_string(),
                    multiple: false,
                },
                EquippedWeapon {
                    weapon: short_sword,
                    hidden: true,
                    equipped_location: "in a sheath hanging from their hip".to_string(),
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
            wearable_type: WearableType::PlateMail,
            material: Some(WearableMaterial::Steel),
            descriptors: vec![WearableDescriptor::SetOf, WearableDescriptor::Drab],
            defense: None,
        };

        let inventory = Inventory {
            equipped_weapons: Vec::new(),
            equipped_wearables: vec![EquippedWearable {
                wearable: chain_mail,
                hidden: false,
                equipped_location: "".to_string(),
                multiple: false,
            }],
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate mail."));
        assert!(!description.contains(", and"));
    }

    #[test]
    fn display_with_two_wearables() {
        let chain_mail = Wearable {
            wearable_type: WearableType::PlateMail,
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
                EquippedWearable {
                    wearable: chain_mail,
                    hidden: false,
                    equipped_location: "".to_string(),
                    multiple: false,
                },
                EquippedWearable {
                    wearable: shackles,
                    hidden: false,
                    equipped_location: "dangling from their wrists".to_string(),
                    multiple: true,
                },
            ],
            carried_weapons: Vec::new(),
            carried_wearables: Vec::new(),
        };

        let description = inventory.to_string();
        assert!(description.contains("set of drab steel plate mail"));
        assert!(description.contains("rusty iron shackles dangling from their wrists."));
        assert!(description.contains(", and"));
    }
}
