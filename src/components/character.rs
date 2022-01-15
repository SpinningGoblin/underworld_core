use std::fmt::Display;

use super::{
    inventory::Inventory, life_modifier::LifeModifier, name::Name, species::Species, stats::Stats,
};

#[derive(Clone, Debug)]
pub struct Character {
    pub name: Option<Name>,
    pub stats: Stats,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub inventory: Option<Inventory>,
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe_species())
    }
}

impl Character {
    fn describe_species(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if let Some(dimensions) = &self.stats.dimensions {
            let height_description = dimensions.describe_height_for_species(&self.species);
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

        let description = character.to_string();
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

        let description = character.to_string();
        assert_eq!("Goblin skeleton", description);
    }
}
