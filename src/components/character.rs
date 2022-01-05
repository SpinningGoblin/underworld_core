use crate::describable::Describable;

use super::{life_modifier::LifeModifier, name::Name, species::Species, stats::Stats};

#[derive(Clone, Debug)]
pub struct Character {
    pub name: Option<Name>,
    pub stats: Stats,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
}

impl Describable for Character {
    fn describe(&self) -> String {
        self.describe_species()
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

        descriptions.push(self.species.describe());

        if let Some(life_modifier) = &self.life_modifier {
            descriptions.push(life_modifier.describe());
        }

        descriptions.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{
            dimensions::Dimensions, life_modifier::LifeModifier, species::Species, stats::Stats,
        },
        describable::Describable,
    };

    use super::Character;

    #[test]
    fn describe_a_character() {
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
        };

        let description = character.describe();
        assert_eq!("tall Goblin", description);
    }

    #[test]
    fn describe_a_character_with_life_modifier() {
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
        };

        let description = character.describe();
        assert_eq!("Goblin skeleton", description);
    }
}
