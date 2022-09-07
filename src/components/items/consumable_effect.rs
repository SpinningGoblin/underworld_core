#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::{Enum, Object};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::components::{
    spells::SpellName,
    {Attack, Defense},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename_all = "snake_case"))]
pub struct ConsumableEffect {
    pub name: ConsumableEffectName,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub learn_spell_effect: Option<LearnSpellEffect>,
    pub healing_effect: Option<HealingEffect>,
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
    oai(rename_all = "snake_case", rename = "ConsumableEffectName")
)]
pub enum ConsumableEffectName {
    LearnSpell,
    HealingGrog,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename_all = "snake_case"))]
pub struct LearnSpellEffect {
    pub spell_name: SpellName,
    pub spell_attack: Option<Attack>,
    pub spell_defense: Option<Defense>,
    pub spell_uses: i32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename_all = "snake_case"))]
pub struct HealingEffect {
    pub healing: Attack,
}
