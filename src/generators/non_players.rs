use rand::Rng;

use crate::components::{character::Character, non_player::NonPlayer};

use super::generator::Generator;

pub struct NonPlayerPrototype {
    pub character_generators: Vec<Box<dyn Generator<Character>>>,
}

impl Generator<NonPlayer> for NonPlayerPrototype {
    fn generate(&self) -> NonPlayer {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.character_generators.len());
        let generator = self.character_generators.get(index).unwrap();
        let character = generator.generate();

        NonPlayer { character }
    }
}
