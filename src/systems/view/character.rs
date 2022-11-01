use crate::components::{
    spells::{LearnedSpellView, SpellMemoryView, SpellView},
    EffectsView, StatsView, {Character, CharacterView, CharacterViewArgs},
};

pub fn view(character: &Character, args: &CharacterViewArgs, knows_all: bool) -> CharacterView {
    let (health, health_known) = if args.knows_health || knows_all {
        (Some(character.stats.health.clone()), true)
    } else {
        (None, false)
    };

    let (inventory, inventory_known) = if args.knows_inventory || knows_all {
        (
            Some(super::inventory::view(
                &character.inventory,
                args.knows_packed_in_inventory,
                knows_all,
            )),
            true,
        )
    } else {
        (None, false)
    };

    let (spell_memory, spell_memory_known) = if knows_all {
        (
            Some(SpellMemoryView {
                spells: character
                    .spell_memory
                    .spells
                    .iter()
                    .map(|learned_spell| LearnedSpellView {
                        id: learned_spell.id.to_string(),
                        spell: SpellView {
                            name: learned_spell.spell.name,
                            attack: learned_spell.spell.attack.clone(),
                            knows_attack: true,
                            defense: learned_spell.spell.defense.clone(),
                            knows_defense: true,
                            uses: learned_spell.spell.uses,
                            knows_uses: true,
                            spell_type: learned_spell.spell.spell_type(),
                        },
                        learned_at: learned_spell.learned_at.to_rfc3339(),
                    })
                    .collect(),
                knows_spells: true,
            }),
            true,
        )
    } else {
        (None, false)
    };

    CharacterView {
        stats: StatsView {
            health,
            health_known,
            height: character.stats.height,
        },
        species: character.species,
        life_modifier: character.life_modifier,
        inventory,
        inventory_known,
        spell_memory,
        spell_memory_known,
        current_effects: EffectsView {
            shield_aura: character.current_effects.shield_aura.clone(),
            knows_has_shield_aura: true,
            retribution_aura: character.current_effects.retribution_aura.clone(),
            knows_has_retribution_aura: true,
            resurrection_aura: character.current_effects.resurrection_aura,
            knows_has_resurrection_aura: true,
            poison: character.current_effects.poison.clone(),
            covered_in_oil: character.current_effects.covered_in_oil,
        },
    }
}
