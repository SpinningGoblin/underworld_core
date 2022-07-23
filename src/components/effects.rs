#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::{Attack, Defense};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Effects {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub shield_aura: Option<Defense>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub retribution_aura: Option<Attack>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub poison: Option<Poison>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub resurrection_aura: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub covered_in_oil: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "openapi",
    derive(Object),
    oai(rename_all = "snake_case", rename = "Effects")
)]
pub struct EffectsView {
    pub shield_aura: Option<Defense>,
    pub knows_has_shield_aura: bool,
    pub retribution_aura: Option<Attack>,
    pub knows_has_retribution_aura: bool,
    pub resurrection_aura: bool,
    pub knows_has_resurrection_aura: bool,
    pub poison: Option<Poison>,
    pub covered_in_oil: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Poison {
    pub damage: i32,
    pub duration: i32,
}
