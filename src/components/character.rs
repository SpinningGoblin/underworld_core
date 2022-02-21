#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
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

pub struct CharacterViewArgs {
    pub knows_health: bool,
    pub knows_species: bool,
    pub knows_life_modifier: bool,
    pub knows_inventory: bool,
    pub knows_hidden_in_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl Character {
    pub fn look_at(&self, args: CharacterViewArgs, knows_all: bool) -> CharacterView {
        let (health, health_known) = if args.knows_health || knows_all {
            (self.stats.health.clone(), true)
        } else {
            (None, false)
        };

        let (species, species_known) = if args.knows_species || knows_all {
            (Some(self.species.clone()), true)
        } else {
            (None, false)
        };

        let (life_modifier, life_modifier_known) = if args.knows_life_modifier || knows_all {
            (self.life_modifier.clone(), true)
        } else {
            (None, false)
        };

        let (inventory, inventory_known) = if args.knows_inventory || knows_all {
            (
                self.inventory.clone().map(|inv| {
                    inv.look_at(
                        args.knows_hidden_in_inventory,
                        args.knows_packed_in_inventory,
                        knows_all,
                    )
                }),
                true,
            )
        } else {
            (None, false)
        };

        CharacterView {
            stats: StatsView {
                health,
                health_known,
                height: self.stats.height.clone(),
            },
            species,
            species_known,
            life_modifier,
            life_modifier_known,
            inventory,
            inventory_known,
        }
    }

    pub fn describe_species(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if !self.stats.height.is_average() {
            descriptions.push(format!("{}", self.stats.height));
        }

        if let Some(life_modifier) = &self.life_modifier {
            descriptions.push(life_modifier.to_string());
        }

        descriptions.push(self.species.to_string());
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
    use crate::components::{life_modifier::LifeModifier, species::Species, stats::Stats};

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

        let description = character.describe_species();
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

        let description = character.describe_species();
        assert_eq!("goblin skeleton", description);
    }
}
