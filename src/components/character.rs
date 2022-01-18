#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use super::{
    inventory::Inventory, life_modifier::LifeModifier, name::Name, species::Species, stats::Stats,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct Character {
    pub name: Option<Name>,
    pub stats: Stats,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
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

    pub fn describe_inventory(&self) -> String {
        match &self.inventory {
            Some(inventory) => format!("{}", inventory),
            _ => "".to_string(),
        }
    }

    pub fn describe_name(&self) -> String {
        match &self.name {
            Some(name) => format!("It says its name is {}", name),
            _ => "It has no name.".to_string(),
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
            name: None,
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
        assert_eq!("tall Goblin", description);
    }

    #[test]
    fn to_string_for_a_character_with_life_modifier() {
        let character = Character {
            name: None,
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
        assert_eq!("Goblin skeleton", description);
    }
}
