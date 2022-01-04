pub mod name;
pub mod player;
pub mod non_player;

use self::name::Name;

use super::stats::Stats;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: Option<Name>,
    pub stats: Stats,
}
