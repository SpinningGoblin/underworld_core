use crate::components::stats::Stats;

use super::Character;

#[derive(Clone, Debug)]
pub struct NonPlayer {
    pub character: Character,
    pub stats: Stats,
}
