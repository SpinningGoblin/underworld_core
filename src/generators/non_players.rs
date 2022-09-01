use uuid::Uuid;

use crate::components::NonPlayer;

use super::{generator::Generator, name::generate_name, CharacterGeneratorBuilder};

#[derive(Default, Clone)]
pub struct NonPlayerGeneratorBuilder {
    name: Option<String>,
    character_gen_builder: Option<CharacterGeneratorBuilder>,
    danger_level: Option<u32>,
}

impl NonPlayerGeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());

        self
    }

    pub fn character_gen_builder(&mut self, builder: CharacterGeneratorBuilder) -> &mut Self {
        self.character_gen_builder = Some(builder);

        self
    }

    pub fn danger_level(&mut self, danger_level: u32) -> &mut Self {
        self.danger_level = Some(danger_level);

        self
    }

    pub fn build(&self) -> impl Generator<NonPlayer> {
        let danger_level = self.danger_level.unwrap_or(1);

        let character_gen_builder = match &self.character_gen_builder {
            Some(builder) => builder.to_owned(),
            None => CharacterGeneratorBuilder::default()
                .danger_level(danger_level)
                .to_owned(),
        };

        let name = if self.name.is_none() {
            generate_name()
        } else {
            self.name.clone()
        };

        NonPlayerPrototype {
            name,
            character_gen_builder,
        }
    }
}

struct NonPlayerPrototype {
    pub name: Option<String>,
    pub character_gen_builder: CharacterGeneratorBuilder,
}

impl Generator<NonPlayer> for NonPlayerPrototype {
    fn generate(&self) -> NonPlayer {
        let character = self.character_gen_builder.build().generate();

        NonPlayer {
            character,
            id: Uuid::new_v4(),
            name: self.name.clone(),
        }
    }
}
