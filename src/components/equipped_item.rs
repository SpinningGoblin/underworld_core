#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct EquippedItem<T: Display + Clone + Debug> {
    pub item: T,
    pub hidden: bool,
    pub equipped_location: String,
    pub multiple: bool,
}
