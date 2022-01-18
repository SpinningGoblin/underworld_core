#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
pub struct Defense {
    pub minimum: i32,
    pub maximum: i32,
}
