#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use super::character::Character;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct NonPlayer {
    pub character: Character,
}
