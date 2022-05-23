use crate::components::{
    character::{Character, CharacterView, CharacterViewArgs},
    spells::{learned_spell::LearnedSpellView, spell::SpellView, spell_memory::SpellMemoryView},
    stats::StatsView,
};

pub fn view(character: &Character, args: &CharacterViewArgs, knows_all: bool) -> CharacterView {
    let (health, health_known) = if args.knows_health || knows_all {
        (character.stats.health.clone(), true)
    } else {
        (None, false)
    };

    let (species, species_known) = if args.knows_species || knows_all {
        (Some(character.species.clone()), true)
    } else {
        (None, false)
    };

    let (life_modifier, life_modifier_known) = if args.knows_life_modifier || knows_all {
        (character.life_modifier.clone(), true)
    } else {
        (None, false)
    };

    let (inventory, inventory_known) = if args.knows_inventory || knows_all {
        (
            character.inventory.clone().map(|inv| {
                super::inventory::view(
                    &inv,
                    args.knows_hidden_in_inventory,
                    args.knows_packed_in_inventory,
                    knows_all,
                )
            }),
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
                        identifier: super::identifier::view(&learned_spell.identifier, true),
                        spell: SpellView {
                            name: learned_spell.spell.name.clone(),
                            attack: learned_spell.spell.attack.clone(),
                            knows_attack: true,
                            defense: learned_spell.spell.defense.clone(),
                            knows_defense: true,
                            uses: learned_spell.spell.uses,
                            knows_uses: true,
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
            height: character.stats.height.clone(),
        },
        species,
        species_known,
        life_modifier,
        life_modifier_known,
        inventory,
        inventory_known,
        spell_memory,
        spell_memory_known,
    }
}
