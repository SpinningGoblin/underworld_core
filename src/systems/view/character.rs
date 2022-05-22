use crate::components::{
    character::{Character, CharacterView, CharacterViewArgs},
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
    }
}
