#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{LearnedSpell, LearnedSpellView};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct SpellMemory {
    pub spells: Vec<LearnedSpell>,
}

impl SpellMemory {
    pub fn find_spell(&self, spell_id: &Uuid) -> Option<&LearnedSpell> {
        self.spells
            .iter()
            .find(|learned_spell| learned_spell.id.eq(spell_id))
    }

    pub fn add_spell(&mut self, learned_spell: LearnedSpell) {
        self.spells.push(learned_spell);
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "SpellMemory"))]
pub struct SpellMemoryView {
    pub spells: Vec<LearnedSpellView>,
    pub knows_spells: bool,
}
