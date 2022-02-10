use enum_iterator::IntoEnumIterator;
use rand::Rng;

use crate::{
    components::{
        fixtures::fixture_type::FixtureType,
        life_modifier::LifeModifier,
        non_player::NonPlayer,
        rooms::{
            group_descriptor::GroupDescriptor, npc_position::NpcPosition,
            npc_position_descriptor::NpcPositionDescriptor, room_type::RoomType,
        },
        species::Species,
        weapons::weapon_type::WeaponType,
        wearables::wearable_type::WearableType,
    },
    generators::{
        characters::CharacterPrototype, generator::Generator, inventory::InventoryPrototype,
        non_players::NonPlayerPrototype,
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
            let life_modifier = life_modifier();
            let mut species = starter_species.clone();
            let mut prototype = npc_prototype(&starter_species, life_modifier.clone());
            let npcs: Vec<NonPlayer> = (0..group_size)
                .map(|index| {
                    if index > 0 {
                        species = switch_species(&species);
                        prototype = npc_prototype(&species, life_modifier.clone());
                    }
                    prototype.generate()
                })
                .collect();

            NpcPosition {
                group_descriptor: group_descriptor(npcs.len()),
                position_descriptor: position_descriptor(npcs.len(), &fixtures_in_room),
                npcs,
            }
        })
        .collect()
}

fn num_groups(room_type: &RoomType) -> usize {
    let range = match *room_type {
        RoomType::Cave => 1..=2,
        RoomType::Cavern => 1..=2,
        RoomType::PrisonCell => 0..=1,
        RoomType::Room => 1..=1,
        RoomType::EntryWay => 0..=1,
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
        Species::Goblin => vec![Species::Goblin, Species::Hobgoblin],
        Species::Hobgoblin => vec![Species::Goblin, Species::Hobgoblin],
        Species::Kobold => vec![Species::Kobold, Species::Bugbear],
        Species::Ogre => vec![Species::Ogre],
        Species::Orc => vec![Species::Orc, Species::Bugbear],
        Species::Unknown => vec![Species::Unknown],
    };

    let index = rng.gen_range(0..choices.len());
    choices.get(index).cloned().unwrap_or_else(|| species.clone())
}

fn choose_species() -> Species {
    let all_species: Vec<Species> = Species::into_enum_iter().collect();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..all_species.len());
    all_species.get(index).cloned().unwrap_or(Species::Unknown)
}

fn group_size(species: &Species) -> usize {
    let range = match *species {
        Species::Bugbear => 1..=2,
        Species::Goblin => 1..=3,
        Species::Hobgoblin => 1..=2,
        Species::Kobold => 1..=3,
        Species::Ogre => 1..=1,
        Species::Orc => 1..=1,
        Species::Unknown => 1..=1,
    };

    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

fn group_descriptor(group_size: usize) -> Option<GroupDescriptor> {
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
        ]
    } else {
        vec![
            NpcPositionDescriptor::AreLeaningAgainstTheTable,
            NpcPositionDescriptor::StandsOnTheTable,
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

fn life_modifier() -> Option<LifeModifier> {
    let mut rng = rand::thread_rng();
    let roll: usize = rng.gen_range(0..=100);

    if roll < UNDEAD_CHANCE {
        None
    } else {
        let type_roll: usize = rng.gen_range(0..=100);
        match type_roll {
            0..=33 => Some(LifeModifier::Skeleton),
            34..=66 => Some(LifeModifier::Vampire),
            _ => Some(LifeModifier::Zombie),
        }
    }
}

fn npc_prototype(species: &Species, life_modifier: Option<LifeModifier>) -> NonPlayerPrototype {
    let inventory_prototype = InventoryPrototype {
        weapon_types: WeaponType::into_enum_iter().collect(),
        wearable_types: WearableType::into_enum_iter().collect(),
        num_equipped_weapons: 0..=2,
        num_equipped_wearables: 0..=5,
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
        name: None,
        character_generator: Box::new(character_prototype),
    }
}
