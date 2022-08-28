use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    components::{
        fixtures::FixtureType,
        items::ItemType,
        rooms::{GroupDescriptor, NpcPosition, NpcPositionDescriptor, RoomType},
        LifeModifier, Species,
    },
    generators::{
        characters::CharacterPrototype, generator::Generator, inventory::InventoryPrototype,
        name::generate_name, non_players::NonPlayerPrototype,
    },
    utils::rolls::{roll_d100, roll_percent_succeeds},
};

const KEEP_SPECIES_CHANGE: i32 = 90;

pub fn build_npc_positions(
    room_type: &RoomType,
    fixtures_in_room: Vec<FixtureType>,
    danger_level: u32,
) -> Vec<NpcPosition> {
    // Decide how many "groups" I would like in the room.
    let num_groups = num_groups(room_type);

    if num_groups == 0 {
        return Vec::new();
    }

    (0..num_groups)
        .flat_map(|_| {
            // For each group, find a starting race.
            let starter_species = choose_species();
            // Get the group size based on the species.
            let group_size = group_size(&starter_species);
            let life_modifier = life_modifier(&starter_species);
            let mut species = starter_species.clone();
            let mut prototype =
                npc_prototype(&starter_species, life_modifier.clone(), danger_level);

            let mut npc_positions: Vec<NpcPosition> = Vec::new();
            (0..group_size).for_each(|index| {
                if index > 0 {
                    species = switch_species(&species);
                    prototype = npc_prototype(&species, life_modifier.clone(), danger_level);
                }
                let mut npc = prototype.generate();

                let position_descriptor = position_descriptor(&fixtures_in_room);

                if position_descriptor == Some(NpcPositionDescriptor::IsLyingInPoolBlood)
                    && !matches!(&npc.character.species, Species::Phantom | Species::Shadow)
                    && npc.character.life_modifier.is_none()
                {
                    npc.kill();
                }

                npc_positions.push(NpcPosition {
                    group_descriptor: group_descriptor(),
                    npc,
                    position_descriptor,
                });
            });

            npc_positions
        })
        .collect()
}

fn num_groups(room_type: &RoomType) -> usize {
    let range = match *room_type {
        RoomType::PrisonCell => 0..=1,
        RoomType::Room => 1..=1,
        RoomType::EntryWay => 0..=1,
        _ => 1..=2,
    };
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

fn switch_species(species: &Species) -> Species {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, KEEP_SPECIES_CHANGE) {
        return species.clone();
    }

    let choices = match *species {
        Species::Bugbear => vec![Species::Kobold, Species::Bugbear, Species::Orc],
        Species::Goblin | Species::Hobgoblin => vec![Species::Goblin, Species::Hobgoblin],
        Species::Kobold => vec![Species::Kobold, Species::Bugbear],
        Species::Orc => vec![Species::Orc, Species::Bugbear],
        Species::Frogkin | Species::Lizardkin | Species::Turtlekin => {
            vec![Species::Lizardkin, Species::Frogkin, Species::Turtlekin]
        }
        _ => vec![species.clone()],
    };

    let index = rng.gen_range(0..choices.len());
    choices
        .get(index)
        .cloned()
        .unwrap_or_else(|| species.clone())
}

fn choose_species() -> Species {
    let all_species: Vec<Species> = Species::iter().collect();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..all_species.len());
    all_species.get(index).cloned().unwrap_or(Species::Shadow)
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

pub fn group_descriptor() -> Option<GroupDescriptor> {
    let options = single_group_descriptors();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..options.len());
    options.get(index).cloned()
}

fn single_group_descriptors() -> Vec<GroupDescriptor> {
    vec![
        GroupDescriptor::A,
        GroupDescriptor::ALone,
        GroupDescriptor::ASingle,
    ]
}

fn position_descriptor(used_fixtures: &[FixtureType]) -> Option<NpcPositionDescriptor> {
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
    options.append(&mut other_positions());

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..options.len());
    options.get(index).cloned()
}

fn other_positions() -> Vec<NpcPositionDescriptor> {
    vec![
        NpcPositionDescriptor::IsGlaringAtYou,
        NpcPositionDescriptor::IsGlaringAtYouFromNearby,
        NpcPositionDescriptor::InCornerStands,
        NpcPositionDescriptor::IsStandingAround,
        NpcPositionDescriptor::IsLyingInPoolBlood,
        NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom,
    ]
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

fn life_modifier(species: &Species) -> Option<LifeModifier> {
    let mut rng = rand::thread_rng();
    if matches!(species, &Species::Phantom | &Species::Shadow) {
        return None;
    }

    if roll_percent_succeeds(&mut rng, UNDEAD_CHANCE) {
        let type_roll = roll_d100(&mut rng, 1, 0);
        match type_roll {
            0..=33 => Some(LifeModifier::Skeleton),
            34..=66 => Some(LifeModifier::Vampire),
            _ => Some(LifeModifier::Zombie),
        }
    } else {
        None
    }
}

fn npc_prototype(
    species: &Species,
    life_modifier: Option<LifeModifier>,
    danger_level: u32,
) -> NonPlayerPrototype {
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

    let inventory_prototype = InventoryPrototype {
        danger_level,
        item_types: ItemType::iter().collect(),
        num_equipped_weapons,
        num_equipped_wearables,
        hidden_weapon_chance: 0,
        hidden_wearable_chance: 0,
    };

    let character_prototype = CharacterPrototype {
        species: species.clone(),
        inventory_generator: Box::new(inventory_prototype),
        life_modifier,
        has_inventory: true,
        danger_level,
    };

    NonPlayerPrototype {
        name: generate_name(),
        character_generator: Box::new(character_prototype),
    }
}
