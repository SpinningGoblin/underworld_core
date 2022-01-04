use crate::{describable::Describable, components::stats::Stats};

use super::Character;

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Character,
    pub stats: Stats,
}

impl Describable for Player {
    fn describe(&self) -> String {
        todo!()
    }
}
