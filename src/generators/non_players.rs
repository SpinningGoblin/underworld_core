use uuid::Uuid;

use crate::components::{
    character::Character, identifier::Identifier, non_player::NonPlayer,
};

use super::{characters::random_character_generator, generator::Generator};

pub fn npc_generator(name: Option<String>) -> impl Generator<NonPlayer> {
    NonPlayerPrototype {
        name,
        character_generator: Box::new(random_character_generator()),
    }
}

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
                id: Uuid::new_v4(),
                name: self.name.clone(),
            },
        }
    }
}
