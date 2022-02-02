pub mod equip_location_descriptor;
pub mod equipped_item;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::utils::sentences::{SentenceJoiners, SentenceStarters};

use self::equipped_item::EquippedItem;

use super::{item::Item, weapons::weapon::Weapon, wearables::wearable::Wearable};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Equipped {
    pub weapons: Vec<EquippedItem<Weapon>>,
    pub wearables: Vec<EquippedItem<Wearable>>,
}

impl Equipped {
    pub fn weapon_description(&self, starter: &str) -> String {
        let visible_weapons: Vec<&EquippedItem<Weapon>> = self
            .weapons
            .iter()
            .filter(|weapon| !weapon.hidden)
            .collect();
        if visible_weapons.is_empty() {
            return format!("{} has no visible weapons", starter);
        }

        let starters = SentenceStarters::weapon_starters();
        let joiners = SentenceJoiners::weapon_joiners();
        let mut weapons: Vec<String> = vec![format!("{} ", starter)];

        for (index, weapon) in visible_weapons.iter().enumerate() {
            if index == 0 {
                weapons.push(format!("{} ", starters.get_starter(weapon.multiple)));
            }

            let description = format!("{} {}", weapon.item.look_at(true), weapon.equipped_location)
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
                    joiners.get_joiner(weapon.multiple),
                    description
                ));
            }
        }

        weapons.push(".".to_string());
        weapons.join("")
    }

    pub fn wearables_description(&self, starter: &str) -> String {
        let visible_wearables: Vec<&EquippedItem<Wearable>> = self
            .wearables
            .iter()
            .filter(|equipped_wearable| !equipped_wearable.hidden)
            .collect();

        if visible_wearables.is_empty() {
            return format!("{} is wearing... nothing?", starter);
        }

        let starters = SentenceStarters::wearable_starters();
        let joiners = SentenceJoiners::wearable_joiners();
        let mut wearables: Vec<String> = vec![format!("{} is ", starter)];

        for (index, wearable) in visible_wearables.iter().enumerate() {
            if index == 0 {
                wearables.push(format!("{} ", starters.get_starter(wearable.multiple)));
            }

            let description = format!(
                "{} {}",
                wearable.item.look_at(true),
                wearable.equipped_location
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
                    joiners.get_joiner(wearable.multiple),
                    description
                ));
            }
        }

        wearables.push(".".to_string());

        wearables.join("")
    }
}
