#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use super::{character::Character, identifier::Identifier};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct Player {
    pub character: Character,
    pub player_name: Identifier,
}
