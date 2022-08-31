#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    damage::AttackEffect,
    items::CharacterItem,
    spells::{
        LearnedSpell, {SpellMemory, SpellMemoryView},
    },
    Attack, Defense, LifeModifier, Species, {Effects, EffectsView}, {Inventory, InventoryView},
    {Stats, StatsView},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Character {
    pub stats: Stats,
    pub species: Species,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub life_modifier: Option<LifeModifier>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub inventory: Inventory,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub current_effects: Effects,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub spell_memory: SpellMemory,
}

impl Character {
    pub fn is_dead(&self) -> bool {
        self.stats.health.current == 0
    }

    pub fn find_item(&self, item_id: &Uuid) -> Option<CharacterItem> {
        self.inventory.find_item(item_id)
    }

    pub fn find_spell(&self, spell_id: &Uuid) -> Option<&LearnedSpell> {
        self.spell_memory.find_spell(spell_id)
    }

    pub fn find_spell_mut(&mut self, spell_id: &Uuid) -> Option<&mut LearnedSpell> {
        self.spell_memory
            .spells
            .iter_mut()
            .find(|spell| spell.id.eq(spell_id))
    }

    pub fn remove_item(&mut self, item_id: &Uuid) -> Option<CharacterItem> {
        self.inventory.remove_item(item_id)
    }

    pub fn forget_spell(&mut self, spell_id: &Uuid) {
        if let Some(index) =
            self.spell_memory
                .spells
                .iter()
                .enumerate()
                .find_map(|(i, learned_spell)| {
                    if learned_spell.id.eq(spell_id) {
                        Some(i)
                    } else {
                        None
                    }
                })
        {
            self.spell_memory.spells.remove(index);
        }
    }

    pub fn add_item(&mut self, character_item: CharacterItem) {
        self.inventory.add_item(character_item)
    }

    pub fn get_current_health(&self) -> i32 {
        self.stats.health.current
    }

    pub fn damage(&mut self, damage: i32) {
        self.stats.health.current -= damage;
    }

    pub fn heal(&mut self, damage_healed: i32) {
        self.stats.health.current += damage_healed;
    }

    pub fn heal_to_max(&mut self) {
        self.stats.health.current = self.stats.health.max;
    }

    pub fn increase_max_health(&mut self, change: i32) {
        self.stats.health.max += change;
        self.stats.health.current += change;
    }

    pub fn kill(&mut self) {
        self.stats.health.current = 0;
    }

    pub fn has_weapons_readied(&self) -> bool {
        !self.inventory.readied_weapons().is_empty()
    }

    pub fn count_weapons_at_ready(&self) -> usize {
        self.inventory.count_weapons_at_ready()
    }

    pub fn count_wearables_at_ready(&self) -> usize {
        self.inventory.count_wearables_at_ready()
    }

    pub fn strongest_non_readied_weapon(&self) -> Option<&CharacterItem> {
        self.inventory.strongest_non_readied_weapon()
    }

    pub fn full_attack(&self) -> Attack {
        let base_attack = self.stats.base_attack.clone().unwrap_or_default();
        let inventory_full_attack = self.inventory.full_attack().unwrap_or_default();

        let mut effects: Vec<AttackEffect> = base_attack
            .effects
            .into_iter()
            .chain(inventory_full_attack.effects.into_iter())
            .collect();

        effects.sort();
        effects.dedup();

        Attack {
            num_rolls: inventory_full_attack.num_rolls + base_attack.num_rolls,
            modifier: inventory_full_attack.modifier + base_attack.modifier,
            effects,
        }
    }

    pub fn full_defense(&self) -> Defense {
        let Defense {
            damage_resistance: inventory_resistance,
        } = self.inventory.full_defense().unwrap_or_default();
        let Defense {
            damage_resistance: base_resistance,
        } = self
            .stats
            .base_damage_resistance
            .clone()
            .unwrap_or_default();

        Defense {
            damage_resistance: base_resistance + inventory_resistance,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Character"))]
pub struct CharacterView {
    pub stats: StatsView,
    pub species: Species,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub life_modifier: Option<LifeModifier>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub inventory: Option<InventoryView>,
    pub inventory_known: bool,
    pub current_effects: EffectsView,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub spell_memory: Option<SpellMemoryView>,
    pub spell_memory_known: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct CharacterViewArgs {
    pub knows_health: bool,
    pub knows_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl CharacterViewArgs {
    pub fn knows_all_args() -> CharacterViewArgs {
        CharacterViewArgs {
            knows_health: true,
            knows_inventory: true,
            knows_packed_in_inventory: true,
        }
    }
}
