#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    effects::{Effects, EffectsView},
    inventory::{Inventory, InventoryView},
    items::character_item::CharacterItem,
    life_modifier::LifeModifier,
    species::Species,
    spells::{
        learned_spell::LearnedSpell,
        spell_memory::{SpellMemory, SpellMemoryView},
    },
    stats::{Stats, StatsView},
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
        if let Some(health) = &self.stats.health {
            health.current == 0
        } else {
            true
        }
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
            .find(|spell| spell.identifier.id.eq(spell_id))
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
                    if learned_spell.identifier.id.eq(spell_id) {
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

    pub fn get_current_health(&self) -> Option<i32> {
        self.stats.health.as_ref().map(|health| health.current)
    }

    pub fn damage(&mut self, damage: i32) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current -= damage;
        }
    }

    pub fn heal(&mut self, damage_healed: i32) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current += damage_healed;
        }
    }

    pub fn heal_to_max(&mut self) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current = health.max;
        }
    }

    pub fn kill(&mut self) {
        if let Some(mut health) = self.stats.health.as_mut() {
            health.current = 0;
        }
    }

    pub fn no_weapons_equipped(&self) -> bool {
        self.inventory.equipped_weapons().is_empty()
    }

    pub fn no_weapons_readied(&self) -> bool {
        self.inventory.readied_weapons().is_empty()
    }

    pub fn count_weapons_at_ready(&self) -> usize {
        self.inventory.count_weapons_at_ready()
    }

    pub fn strongest_non_readied_weapon(&self) -> Option<&CharacterItem> {
        self.inventory.strongest_non_readied_weapon()
    }

    pub fn strongest_unequipped_weapon(&self) -> Option<&CharacterItem> {
        self.inventory.strongest_unequipped_weapon()
    }

    pub fn attack(&self) -> i32 {
        let mut rng = rand::thread_rng();

        self.inventory
            .equipment
            .iter()
            .filter(|character_item| character_item.at_the_ready)
            .map(|character_item| {
                character_item
                    .item
                    .attack
                    .as_ref()
                    .map(|attack| attack.attack_roll(&mut rng))
                    .unwrap_or_default()
            })
            .sum()
    }

    pub fn defense(&self) -> i32 {
        self.inventory
            .equipment
            .iter()
            .map(|character_item| {
                character_item
                    .item
                    .defense
                    .as_ref()
                    .map(|defense| defense.damage_resistance)
                    .unwrap_or_default()
            })
            .sum()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Character"))]
pub struct CharacterView {
    pub stats: StatsView,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub species: Option<Species>,
    pub species_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub life_modifier: Option<LifeModifier>,
    pub life_modifier_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub inventory: Option<InventoryView>,
    pub inventory_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub current_effects: Option<EffectsView>,
    pub current_effects_known: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub spell_memory: Option<SpellMemoryView>,
    pub spell_memory_known: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct CharacterViewArgs {
    pub knows_health: bool,
    pub knows_species: bool,
    pub knows_life_modifier: bool,
    pub knows_inventory: bool,
    pub knows_hidden_in_inventory: bool,
    pub knows_packed_in_inventory: bool,
}

impl CharacterView {
    pub fn describe_species(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();

        if !self.stats.height.is_average() {
            descriptions.push(format!("{}", self.stats.height));
        }

        if self.life_modifier_known {
            if let Some(life_modifier) = &self.life_modifier {
                descriptions.push(format!("{}", life_modifier));
            }
        }

        if self.species_known {
            if let Some(species) = &self.species {
                descriptions.push(format!("{}", species));
            }
        }

        descriptions.join(" ")
    }

    pub fn describe_inventory(&self, starter: &str) -> String {
        let sentence_starter = if starter.is_empty() {
            format!("The {}", self.describe_species())
        } else {
            format!("{} {}", starter, self.describe_species())
        };

        match &self.inventory {
            Some(inventory) => inventory.describe(&sentence_starter, "It"),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{
            character::CharacterViewArgs, effects::Effects, inventory::Inventory,
            life_modifier::LifeModifier, species::Species, spells::spell_memory::SpellMemory,
            stats::Stats,
        },
        systems::view::character::view,
    };

    use super::Character;

    #[test]
    fn to_string_for_a_character() {
        let character = Character {
            stats: Stats {
                health: None,
                height: crate::components::size::Size::Tall,
            },
            species: Species::Goblin,
            life_modifier: None,
            inventory: Inventory::default(),
            current_effects: Effects::default(),
            spell_memory: SpellMemory::default(),
        };

        let description = view(&character, &CharacterViewArgs::default(), true).describe_species();
        assert_eq!("tall goblin", description);
    }

    #[test]
    fn to_string_for_a_character_with_life_modifier() {
        let character = Character {
            stats: Stats {
                health: None,
                height: crate::components::size::Size::Average,
            },
            species: Species::Goblin,
            life_modifier: Some(LifeModifier::Skeleton),
            inventory: Inventory::default(),
            current_effects: Effects::default(),
            spell_memory: SpellMemory::default(),
        };

        let description = view(&character, &CharacterViewArgs::default(), true).describe_species();
        assert_eq!("skeleton goblin", description);
    }
}
