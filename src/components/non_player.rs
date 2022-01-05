use super::{character::Character, stats::Stats};

#[derive(Clone, Debug)]
pub struct NonPlayer {
    pub character: Character,
    pub stats: Stats,
}
