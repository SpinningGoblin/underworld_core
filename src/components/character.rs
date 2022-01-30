#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{inventory::Inventory, life_modifier::LifeModifier, species::Species, stats::Stats};

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
    pub fn describe_species(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if let Some(dimensions) = &self.stats.dimensions {
            let height_description = dimensions.describe_height(&self.species);
            if !height_description.is_empty() {
                descriptions.push(height_description);
            }
        }

        descriptions.push(self.species.to_string());

        if let Some(life_modifier) = &self.life_modifier {
            descriptions.push(life_modifier.to_string());
        }

        descriptions.join(" ")
    }

    pub fn describe_inventory(&self, starter: &str) -> String {
        let sentence_starter = if starter.is_empty() {
            format!("The {}", self.species)
        } else {
            format!("{} {}", starter, self.species)
        };

        match &self.inventory {
            Some(inventory) => inventory.describe(&sentence_starter, "It"),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        dimensions::Dimensions, life_modifier::LifeModifier, species::Species, stats::Stats,
    };

    use super::Character;

    #[test]
    fn to_string_for_a_character() {
        let character = Character {
            stats: Stats {
                health: None,
                dimensions: Some(Dimensions {
                    height: 2.0,
                    width: 1.0,
                }),
            },
            species: Species::Goblin,
            life_modifier: None,
            inventory: None,
        };

        let description = character.describe_species();
        assert_eq!("tall goblin", description);
    }

    #[test]
    fn to_string_for_a_character_with_life_modifier() {
        let character = Character {
            stats: Stats {
                health: None,
                dimensions: Some(Dimensions {
                    height: 1.0,
                    width: 1.0,
                }),
            },
            species: Species::Goblin,
            life_modifier: Some(LifeModifier::Skeleton),
            inventory: None,
        };

        let description = character.describe_species();
        assert_eq!("goblin skeleton", description);
    }
}
