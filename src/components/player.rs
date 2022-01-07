use super::{character::Character, stats::Stats};

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Character,
    pub stats: Stats,
}
