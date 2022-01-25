use crate::components::{character::Character, identifier::Identifier, non_player::NonPlayer};

use super::generator::Generator;

pub struct NonPlayerPrototype {
    pub name: Option<String>,
    pub character_generator: Box<dyn Generator<Character>>,
}

impl Generator<NonPlayer> for NonPlayerPrototype {
    fn generate(&self) -> NonPlayer {
        let character = self.character_generator.generate();

        NonPlayer {
            character,
            identifier: Identifier {
                name: self.name.clone(),
            },
        }
    }
}
