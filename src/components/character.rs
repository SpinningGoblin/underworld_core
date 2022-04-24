#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{
    inventory::{Inventory, InventoryView},
    life_modifier::LifeModifier,
    species::Species,
    stats::{Stats, StatsView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Character {
    pub stats: Stats,
    pub species: Species,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub life_modifier: Option<LifeModifier>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub inventory: Option<Inventory>,
}

impl Character {
    pub fn is_dead(&self) -> bool {
        match &self.stats.health {
            Some(health) => health.current == 0,
            None => true,
        }
    }

    pub fn get_current_health(&self) -> Option<i32> {
        self.stats.health.as_ref().map(|health| health.current)
    }

    pub fn damage(&mut self, damage: i32) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current -= damage;
        }
    }

    pub fn kill(&mut self) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current = 0;
        }
    }

    pub fn attack(&self) -> i32 {
        let mut rng = rand::thread_rng();

        match &self.inventory {
            Some(inventory) => inventory
                .equipment
                .iter()
                .map(|character_item| {
                    character_item
                        .item
                        .attack
                        .as_ref()
                        .map(|attack| attack.attack_roll(&mut rng))
                        .unwrap_or_default()
                })
                .sum(),
            None => 0,
        }
    }

    pub fn defense(&self) -> i32 {
        match &self.inventory {
            Some(inventory) => inventory
                .equipment
                .iter()
                .map(|character_item| {
                    character_item
                        .item
                        .defense
                        .as_ref()
                        .map(|defense| defense.damage_resistance)
                        .unwrap_or_default()
                })
                .sum(),
            None => 0,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct CharacterView {
    pub stats: StatsView,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub species: Option<Species>,
    pub species_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub life_modifier: Option<LifeModifier>,
    pub life_modifier_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub inventory: Option<InventoryView>,
    pub inventory_known: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CharacterViewArgs {
    pub knows_health: bool,
    pub knows_species: bool,
    pub knows_life_modifier: bool,
    pub knows_inventory: bool,
    pub knows_hidden_in_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl CharacterView {
    pub fn describe_species(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if !self.stats.height.is_average() {
            descriptions.push(format!("{}", self.stats.height));
        }

        if self.life_modifier_known {
            if let Some(life_modifier) = &self.life_modifier {
                descriptions.push(format!("{}", life_modifier));
            }
        }

        if self.species_known {
            if let Some(species) = &self.species {
                descriptions.push(format!("{}", species));
            }
        }

        descriptions.join(" ")
    }

    pub fn describe_inventory(&self, starter: &str) -> String {
        let sentence_starter = if starter.is_empty() {
            format!("The {}", self.describe_species())
        } else {
            format!("{} {}", starter, self.describe_species())
        };

        match &self.inventory {
            Some(inventory) => inventory.describe(&sentence_starter, "It"),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{
            character::CharacterViewArgs, life_modifier::LifeModifier, species::Species,
            stats::Stats,
        },
        systems::view::character::look_at,
    };

    use super::Character;

    #[test]
    fn to_string_for_a_character() {
        let character = Character {
            stats: Stats {
                health: None,
                height: crate::components::size::Size::Tall,
            },
            species: Species::Goblin,
            life_modifier: None,
            inventory: None,
        };

        let description =
            look_at(&character, &CharacterViewArgs::default(), true).describe_species();
        assert_eq!("tall goblin", description);
    }

    #[test]
    fn to_string_for_a_character_with_life_modifier() {
        let character = Character {
            stats: Stats {
                health: None,
                height: crate::components::size::Size::Average,
            },
            species: Species::Goblin,
            life_modifier: Some(LifeModifier::Skeleton),
            inventory: None,
        };

        let description =
            look_at(&character, &CharacterViewArgs::default(), true).describe_species();
        assert_eq!("skeleton goblin", description);
    }
}
