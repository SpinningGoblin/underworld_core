use rand::Rng;
use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{identifier::Identifier, non_player::NonPlayer, species::Species};

use super::{
    descriptor::Descriptor, dimensions::Dimensions, fixture_position::FixturePosition,
    flavour::Flavour, npc_position::NpcPosition, room_type::RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Room {
    pub identifier: Identifier,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub fixture_positions: Vec<FixturePosition>,
    pub dimensions: Dimensions,
    pub npc_positions: Vec<NpcPosition>,
    pub flavour: Option<Flavour>,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![
            self.describe_room(),
            self.describe_fixtures(),
            self.describe_inhabitants(),
        ];

        write!(f, "{}", parts.join(" "))
    }
}

fn get_count_text(original_count: &usize, current_count: &usize) -> String {
    let difference = original_count - current_count;

    if difference == original_count - 1 {
        "The last".to_string()
    } else if difference == 0 {
        "The first".to_string()
    } else {
        "Another".to_string()
    }
}

impl Room {
    pub fn look_at_inhabitants(&self) -> String {
        let non_players: Vec<&NonPlayer> = self
            .npc_positions
            .iter()
            .flat_map(|npc_position| npc_position.npcs.iter())
            .collect();
        if non_players.is_empty() {
            return "".to_string();
        }

        if non_players.len() == 1 {
            let npc = non_players.get(0).unwrap();
            return npc.look_at("");
        }

        let mut descriptions: Vec<String> = Vec::new();
        let original_species_counts = self.species_counts(&non_players);
        let mut current_counts = original_species_counts.clone();
        for npc in non_players.iter() {
            let original_species_count =
                original_species_counts.get(&npc.character.species).unwrap();
            let current_species_count = &current_counts.get(&npc.character.species).unwrap();
            if original_species_count.eq(&1) {
                descriptions.push(npc.look_at(""));
            } else {
                let starter = get_count_text(original_species_count, current_species_count);
                descriptions.push(npc.look_at(&starter));
                let new_count = *current_species_count - 1;
                current_counts
                    .insert(npc.character.species.clone(), new_count)
                    .unwrap();
            }
        }

        descriptions.join(" ")
    }

    fn describe_room(&self) -> String {
        let mut parts: Vec<String> = vec!["It is a".to_string()];

        let dimensions = format!("{}", &self.dimensions);

        if !dimensions.is_empty() {
            parts.push(dimensions);
        }

        self.descriptors
            .iter()
            .filter(|descriptor| descriptor.is_pre())
            .for_each(|descriptor| parts.push(format!(" {}", descriptor)));

        parts.push(format!(" {}", &self.room_type));
        parts.push(".".to_string());

        if let Some(flavour) = &self.flavour {
            parts.push(format!(" {}", flavour.as_sentence()));
        }

        parts.join("")
    }

    fn species_counts(&self, non_players: &[&NonPlayer]) -> HashMap<Species, usize> {
        non_players.iter().fold(HashMap::new(), |mut acc, npc| {
            let species = npc.character.species.clone();
            *acc.entry(species).or_insert(0) += 1;
            acc
        })
    }

    fn describe_fixtures(&self) -> String {
        if self.fixture_positions.is_empty() {
            return "".to_string();
        }

        if self
            .fixture_positions
            .iter()
            .all(|fixture_position| fixture_position.fixtures.is_empty())
        {
            return "".to_string();
        }

        let all_fixtures: Vec<String> = self
            .fixture_positions
            .iter()
            .map(|fixture_position| fixture_position.display_as_sentence())
            .collect();
        all_fixtures.join(" ")
    }

    fn describe_inhabitants(&self) -> String {
        if self.npc_positions.is_empty() {
            return Self::empty_room_description();
        }

        if self
            .npc_positions
            .iter()
            .all(|npc_position| npc_position.npcs.is_empty())
        {
            return Self::empty_room_description();
        }

        let all_groups: Vec<String> = self
            .npc_positions
            .iter()
            .map(|npc_position| npc_position.display_as_sentence())
            .collect();
        all_groups.join(" ")
    }

    fn empty_room_description() -> String {
        let descriptions = vec![
            "There is no one around.",
            "It is empty.",
            "It is eerily empty.",
            "You are alone in the room.",
            "You see no one else.",
            "All that's here are crickets. Invisible crickets.",
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..descriptions.len());
        match descriptions.get(index) {
            Some(it) => it.to_string(),
            _ => "The room is empty.".to_string(),
        }
    }
}
