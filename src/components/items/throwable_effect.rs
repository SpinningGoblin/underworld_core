#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::{Enum, Object};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename_all = "snake_case"))]
pub struct ThrowableEffect {
    pub name: ThrowableEffectName,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub oil_splash_effect: Option<OilSplashEffect>,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "openapi",
    derive(Enum),
    oai(rename_all = "snake_case", rename = "ThrowableEffectName")
)]
pub enum ThrowableEffectName {
    OilSplash,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct OilSplashEffect {
    pub covers_all_enemies: bool,
}
