#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::SpellType;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum SpellName {
    AcidSplash,
    ElectricBlast,
    GreatHeal,
    Heal,
    Phoenix,
    PoisonCloud,
    PoisonDart,
    QuickHeal,
    RagingFireball,
    Retribution,
    TinyShield,
}

impl SpellName {
    pub fn spell_type(&self) -> SpellType {
        match *self {
            SpellName::ElectricBlast
            | SpellName::RagingFireball
            | SpellName::PoisonCloud
            | SpellName::PoisonDart
            | SpellName::AcidSplash => SpellType::Attack,
            SpellName::Heal | SpellName::QuickHeal | SpellName::GreatHeal => SpellType::Healing,
            SpellName::Phoenix | SpellName::Retribution | SpellName::TinyShield => {
                SpellType::PlayerEffect
            }
        }
    }
}
