use super::{character::Character, name::Name};

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Character,
    pub player_name: Name,
}
