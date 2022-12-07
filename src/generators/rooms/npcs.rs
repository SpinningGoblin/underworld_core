use rand::Rng;
use uuid::Uuid;

use crate::{
    components::{
        fixtures::FixtureType,
        rooms::{NpcPosition, NpcPositionDescriptor},
        LifeModifier, NonPlayer, Species,
    },
    generators::{
        generator::Generator, non_players::NonPlayerGeneratorBuilder, CharacterGeneratorBuilder,
        InventoryGeneratorBuilder,
    },
    utils::rolls::{roll_d100, roll_percent_succeeds},
};

use super::BuildNpcsArgs;

const SPAWN_FROM_GHOST_CHANCE: i32 = 10;

pub fn build_npc_positions(
    fixtures_in_room: Vec<FixtureType>,
    danger_level: u32,
    args: &BuildNpcsArgs,
) -> Vec<NpcPosition> {
    let mut rng = rand::thread_rng();

    // Decide how many "groups" I would like in the room.
    let num_groups = rng.gen_range(args.num_groups.clone());
    if num_groups == 0 {
        return Vec::new();
    }

    let mut ghost_spawned = false;

    (0..num_groups)
        .flat_map(|_| {
            // For each group, find a starting race.
            let starter_species = choose_species(&args.possible_species);
            // Get the group size based on the species.
            let group_size = group_size(&starter_species);
            let life_modifier = life_modifier(&starter_species, &args.possible_life_modifiers);
            let mut species = starter_species;
            let mut prototype = npc_prototype(&species, life_modifier, danger_level);

            let mut npc_positions: Vec<NpcPosition> = Vec::new();
            (0..group_size).for_each(|index| {
                if roll_percent_succeeds(&mut rng, SPAWN_FROM_GHOST_CHANCE)
                    && !args.ghosts.is_empty()
                    && !ghost_spawned
                {
                    let index = rng.gen_range(0..args.ghosts.len());
                    let ghost = args.ghosts.get(index).unwrap();

                    let npc = NonPlayer {
                        character: ghost.character.clone(),
                        id: Uuid::new_v4(),
                        name: ghost.name.clone(),
                    };

                    let position_descriptor = position_descriptor(&fixtures_in_room, false);

                    npc_positions.push(NpcPosition {
                        npc,
                        position_descriptor,
                    });
                    ghost_spawned = true;
                } else {
                    if index > 0 {
                        species = switch_species(&species);
                        prototype = npc_prototype(&species, life_modifier, danger_level);
                    }
                    let mut npc = prototype.generate();

                    let include_dead_spawn_positions = args.allow_npcs_to_spawn_dead
                        && !matches!(&npc.character.species, Species::Phantom | Species::Shadow)
                        && npc.character.life_modifier.is_none();

                    let position_descriptor =
                        position_descriptor(&fixtures_in_room, include_dead_spawn_positions);

                    if position_descriptor == Some(NpcPositionDescriptor::IsLyingInPoolBlood) {
                        npc.kill();
                    }

                    npc_positions.push(NpcPosition {
                        npc,
                        position_descriptor,
                    });
                }
            });

            npc_positions
        })
        .collect()
}

const KEEP_SPECIES_CHANCE: i32 = 90;

fn switch_species(species: &Species) -> Species {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, KEEP_SPECIES_CHANCE) {
        return *species;
    }

    let choices = match *species {
        Species::Bugbear => vec![Species::Kobold, Species::Bugbear, Species::Orc],
        Species::Goblin | Species::Hobgoblin | Species::Moblin => {
            vec![Species::Goblin, Species::Hobgoblin, Species::Moblin]
        }
        Species::Kobold => vec![Species::Kobold, Species::Bugbear],
        Species::Orc => vec![Species::Orc, Species::Bugbear],
        Species::Frogkin | Species::Lizardkin | Species::Turtlekin | Species::Rockoblin => {
            vec![
                Species::Lizardkin,
                Species::Frogkin,
                Species::Turtlekin,
                Species::Rockoblin,
            ]
        }
        Species::Dragonkin => vec![Species::Dragonkin],
        Species::Ogre => vec![Species::Ogre],
        Species::Phantom | Species::Shadow => vec![Species::Phantom, Species::Shadow],
    };

    let index = rng.gen_range(0..choices.len());
    choices.get(index).cloned().unwrap_or(*species)
}

fn choose_species(species: &[Species]) -> Species {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..species.len());
    species.get(index).cloned().unwrap_or(Species::Shadow)
}

fn group_size(species: &Species) -> usize {
    let range = match *species {
        Species::Bugbear | Species::Hobgoblin => 1..=2,
        Species::Goblin | Species::Kobold => 1..=3,
        _ => 1..=1,
    };

    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

fn position_descriptor(
    used_fixtures: &[FixtureType],
    include_dead_spawn_positions: bool,
) -> Option<NpcPositionDescriptor> {
    let mut options: Vec<NpcPositionDescriptor> = Vec::new();

    for fixture_type in used_fixtures {
        let mut fixture_options = match *fixture_type {
            FixtureType::Barrel => barrel_positions(),
            FixtureType::Bed => bed_positions(),
            FixtureType::Chair => chair_positions(),
            FixtureType::Chest => chest_positions(),
            FixtureType::Cot => cot_positions(),
            FixtureType::Crate => crate_positions(),
            FixtureType::SleepingRoll => sleeping_roll_positions(),
            FixtureType::Table => table_positions(),
            FixtureType::WeaponRack => weapon_rack_positions(),
            _ => Vec::new(),
        };

        options.append(&mut fixture_options);
    }
    options.append(&mut other_positions(include_dead_spawn_positions));

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..options.len());
    options.get(index).cloned()
}

fn other_positions(include_dead_spawn_positions: bool) -> Vec<NpcPositionDescriptor> {
    let mut positions = vec![
        NpcPositionDescriptor::IsGlaringAtYou,
        NpcPositionDescriptor::IsGlaringAtYouFromNearby,
        NpcPositionDescriptor::InCornerStands,
        NpcPositionDescriptor::IsStandingAround,
        NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom,
    ];

    if include_dead_spawn_positions {
        positions.append(&mut dead_spawn_positions());
    }

    positions
}

fn dead_spawn_positions() -> Vec<NpcPositionDescriptor> {
    vec![NpcPositionDescriptor::IsLyingInPoolBlood]
}

fn barrel_positions() -> Vec<NpcPositionDescriptor> {
    vec![NpcPositionDescriptor::IsStandingInABarrel]
}

fn bed_positions() -> Vec<NpcPositionDescriptor> {
    vec![]
}

fn chair_positions() -> Vec<NpcPositionDescriptor> {
    vec![
        NpcPositionDescriptor::IsSittingInAChair,
        NpcPositionDescriptor::SittingInAChairIs,
    ]
}

fn chest_positions() -> Vec<NpcPositionDescriptor> {
    vec![
        NpcPositionDescriptor::IsCrouchedOverChest,
        NpcPositionDescriptor::IsRummagingThroughAChest,
    ]
}

fn crate_positions() -> Vec<NpcPositionDescriptor> {
    vec![NpcPositionDescriptor::IsLeaningOnACrate]
}

fn cot_positions() -> Vec<NpcPositionDescriptor> {
    vec![]
}

fn sleeping_roll_positions() -> Vec<NpcPositionDescriptor> {
    vec![]
}

fn table_positions() -> Vec<NpcPositionDescriptor> {
    vec![
        NpcPositionDescriptor::IsStandingOnTheTable,
        NpcPositionDescriptor::IsLeaningAgainstTheTable,
        NpcPositionDescriptor::StandsOnTheTable,
    ]
}

fn weapon_rack_positions() -> Vec<NpcPositionDescriptor> {
    vec![NpcPositionDescriptor::IsLookingAtTheWeaponRack]
}

const UNDEAD_CHANCE: i32 = 15;

fn life_modifier(species: &Species, possible_modifiers: &[LifeModifier]) -> Option<LifeModifier> {
    let mut rng = rand::thread_rng();
    if matches!(species, &Species::Phantom | &Species::Shadow) {
        return None;
    }

    if roll_percent_succeeds(&mut rng, UNDEAD_CHANCE) {
        let type_roll = roll_d100(&mut rng, 1, 0);
        if (0..=33).contains(&type_roll) && possible_modifiers.contains(&LifeModifier::Skeleton) {
            Some(LifeModifier::Skeleton)
        } else if (34..=66).contains(&type_roll)
            && possible_modifiers.contains(&LifeModifier::Vampire)
        {
            Some(LifeModifier::Vampire)
        } else if possible_modifiers.contains(&LifeModifier::Zombie) {
            Some(LifeModifier::Zombie)
        } else {
            None
        }
    } else {
        None
    }
}

fn npc_prototype(
    species: &Species,
    life_modifier: Option<LifeModifier>,
    danger_level: u32,
) -> impl Generator<NonPlayer> {
    let num_equipped_weapons = if (1..=10).contains(&danger_level) {
        1..=1
    } else if (11..=40).contains(&danger_level) {
        1..=2
    } else {
        2..=2
    };

    let num_equipped_wearables = if (1..=10).contains(&danger_level) {
        1..=4
    } else if (11..=20).contains(&danger_level) {
        2..=5
    } else if (21..=40).contains(&danger_level) {
        3..=6
    } else {
        4..=8
    };

    let inventory_generator = InventoryGeneratorBuilder::new()
        .danger_level(danger_level)
        .num_equipped_weapons(num_equipped_weapons)
        .num_equipped_wearables(num_equipped_wearables)
        .to_owned();

    let mut character_gen_builder = CharacterGeneratorBuilder::new()
        .danger_level(danger_level)
        .inventory_generator_builder(inventory_generator)
        .species(*species)
        .to_owned();

    if let Some(modifier) = life_modifier {
        character_gen_builder.life_modifier(modifier);
    }

    let npc_gen_builder = NonPlayerGeneratorBuilder::default()
        .danger_level(danger_level)
        .character_gen_builder(character_gen_builder)
        .to_owned();

    npc_gen_builder.build()
}
