use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::{
    components::{
        fixtures::fixture_type::FixtureType,
        items::item_type::ItemType,
        life_modifier::LifeModifier,
        non_player::NonPlayer,
        rooms::{
            group_descriptor::GroupDescriptor, npc_position::NpcPosition,
            npc_position_descriptor::NpcPositionDescriptor, room_type::RoomType,
        },
        species::Species,
    },
    generators::{
        characters::CharacterPrototype, generator::Generator, inventory::InventoryPrototype,
        name::generate_name, non_players::NonPlayerPrototype,
    },
};

const SWITCH_SPECIES_CHANCE: usize = 10;

pub fn build_npc_positions(
    room_type: &RoomType,
    fixtures_in_room: Vec<FixtureType>,
) -> Vec<NpcPosition> {
    // Decide how many "groups" I would like in the room.
    let num_groups = num_groups(room_type);

    if num_groups == 0 {
        return Vec::new();
    }

    (0..num_groups)
        .map(|_| {
            // For each group, find a starting race.
            let starter_species = choose_species();
            // Get the group size based on the species.
            let group_size = group_size(&starter_species);
            let life_modifier = life_modifier(&starter_species);
            let mut species = starter_species.clone();
            let mut prototype = npc_prototype(&starter_species, life_modifier.clone());
            let mut npcs: Vec<NonPlayer> = (0..group_size)
                .map(|index| {
                    if index > 0 {
                        species = switch_species(&species);
                        prototype = npc_prototype(&species, life_modifier.clone());
                    }
                    prototype.generate()
                })
                .collect();

            let position_descriptor = position_descriptor(npcs.len(), &fixtures_in_room);

            if position_descriptor == Some(NpcPositionDescriptor::IsLyingInPoolBlood) {
                npcs.iter_mut().for_each(|npc| {
                    if !matches!(&npc.character.species, Species::Phantom | Species::Shadow)
                        && npc.character.life_modifier == None
                    {
                        npc.kill();
                    }
                });
            }

            NpcPosition {
                group_descriptor: group_descriptor(npcs.len()),
                position_descriptor,
                npcs,
            }
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
    let switch_roll: usize = rng.gen_range(0..=100);
    if switch_roll > SWITCH_SPECIES_CHANCE {
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
    let all_species: Vec<Species> = Species::into_enum_iter().collect();
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

pub fn group_descriptor(group_size: usize) -> Option<GroupDescriptor> {
    let options = match group_size {
        1 => single_group_descriptors(),
        _ => multi_group_descriptors(),
    };

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

fn multi_group_descriptors() -> Vec<GroupDescriptor> {
    vec![
        GroupDescriptor::Some,
        GroupDescriptor::AFew,
        GroupDescriptor::AGangOf,
        GroupDescriptor::AGroupOf,
    ]
}

fn position_descriptor(
    group_size: usize,
    used_fixtures: &[FixtureType],
) -> Option<NpcPositionDescriptor> {
    let mut options: Vec<NpcPositionDescriptor> = Vec::new();

    for fixture_type in used_fixtures {
        let mut fixture_options = match *fixture_type {
            FixtureType::Barrel => barrel_positions(group_size),
            FixtureType::Bed => bed_positions(group_size),
            FixtureType::Chair => chair_positions(group_size),
            FixtureType::Chest => chest_positions(group_size),
            FixtureType::Cot => cot_positions(group_size),
            FixtureType::Crate => crate_positions(group_size),
            FixtureType::SleepingRoll => sleeping_roll_positions(group_size),
            FixtureType::Table => table_positions(group_size),
            FixtureType::WeaponRack => weapon_rack_positions(group_size),
            _ => Vec::new(),
        };

        options.append(&mut fixture_options);
    }
    options.append(&mut other_positions(group_size));

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..options.len());
    options.get(index).cloned()
}

fn other_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![
            NpcPositionDescriptor::IsGlaringAtYou,
            NpcPositionDescriptor::IsGlaringAtYouFromNearby,
            NpcPositionDescriptor::InCornerStands,
            NpcPositionDescriptor::IsStandingAround,
            NpcPositionDescriptor::IsLyingInPoolBlood,
        ]
    } else {
        vec![
            NpcPositionDescriptor::AreGlaringAtYou,
            NpcPositionDescriptor::AreGlaringAtYouFromNearby,
            NpcPositionDescriptor::AreInTheCorner,
            NpcPositionDescriptor::InCornerStands,
            NpcPositionDescriptor::InTheCornerAre,
        ]
    }
}

fn barrel_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsStandingInABarrel]
    } else {
        Vec::new()
    }
}

fn bed_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsSleepingInTheBed]
    } else {
        Vec::new()
    }
}

fn chair_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![
            NpcPositionDescriptor::IsSittingInAChair,
            NpcPositionDescriptor::SittingInAChairIs,
        ]
    } else {
        vec![NpcPositionDescriptor::AreSittingInChairs]
    }
}

fn chest_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![
            NpcPositionDescriptor::IsCrouchedOverChest,
            NpcPositionDescriptor::IsRummagingThroughAChest,
        ]
    } else {
        Vec::new()
    }
}

fn crate_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsLeaningOnACrate]
    } else {
        vec![NpcPositionDescriptor::AreLeaningOnACrate]
    }
}

fn cot_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsSleepingInACot]
    } else {
        Vec::new()
    }
}

fn sleeping_roll_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsSleepingInSleepingRoll]
    } else {
        Vec::new()
    }
}

fn table_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![
            NpcPositionDescriptor::IsStandingOnTheTable,
            NpcPositionDescriptor::IsLeaningAgainstTheTable,
            NpcPositionDescriptor::StandsOnTheTable,
        ]
    } else {
        vec![
            NpcPositionDescriptor::AreLeaningAgainstTheTable,
            NpcPositionDescriptor::StandOnTheTable,
        ]
    }
}

fn weapon_rack_positions(group_size: usize) -> Vec<NpcPositionDescriptor> {
    if group_size == 1 {
        vec![NpcPositionDescriptor::IsLookingAtTheWeaponRack]
    } else {
        vec![NpcPositionDescriptor::AreLookingAtTheWeaponRack]
    }
}

const UNDEAD_CHANCE: usize = 15;

fn life_modifier(species: &Species) -> Option<LifeModifier> {
    let mut rng = rand::thread_rng();
    let roll: usize = rng.gen_range(0..=100);

    if matches!(species, &Species::Phantom | &Species::Shadow) {
        return None;
    }

    if roll < UNDEAD_CHANCE {
        let type_roll: usize = rng.gen_range(0..=100);
        match type_roll {
            0..=33 => Some(LifeModifier::Skeleton),
            34..=66 => Some(LifeModifier::Vampire),
            _ => Some(LifeModifier::Zombie),
        }
    } else {
        None
    }
}

fn npc_prototype(species: &Species, life_modifier: Option<LifeModifier>) -> NonPlayerPrototype {
    let inventory_prototype = InventoryPrototype {
        item_types: ItemType::into_enum_iter().collect(),
        num_equipped_weapons: 1..=1,
        num_equipped_wearables: 1..=4,
        num_carried_weapons: 0..=1,
        num_carried_wearables: 0..=1,
        hidden_weapon_chance: 0,
        hidden_wearable_chance: 0,
    };

    let character_prototype = CharacterPrototype {
        species: species.clone(),
        inventory_generator: Box::new(inventory_prototype),
        life_modifier,
        has_inventory: true,
    };

    NonPlayerPrototype {
        name: generate_name(),
        character_generator: Box::new(character_prototype),
    }
}
