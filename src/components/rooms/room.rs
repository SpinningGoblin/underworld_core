use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use rand::Rng;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    dimensions::Dimensions, fixtures::fixture::Fixture, identifier::Identifier,
    non_player::NonPlayer, species::Species,
};

use super::{descriptor::Descriptor, room_type::RoomType};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Room {
    pub identifier: Identifier,
    pub dimensions: Dimensions,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub non_players: Vec<NonPlayer>,
    pub fixtures: Vec<Fixture>,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![self.describe_room(), self.describe_inhabitants()];

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
        if self.non_players.is_empty() {
            return "".to_string();
        }

        if self.non_players.len() == 1 {
            let npc = self.non_players.get(0).unwrap();
            return npc.look_at("");
        }

        let mut descriptions: Vec<String> = Vec::new();
        let original_species_counts = self.species_counts();
        let mut current_counts = original_species_counts.clone();
        for npc in self.non_players.iter() {
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

        let height_description = self.dimensions.describe_height(&self.room_type);
        if !height_description.is_empty() {
            parts.push(format!(" {}", height_description));
        }

        let width_description = self.dimensions.describe_width(&self.room_type);
        if !width_description.is_empty() {
            parts.push(format!(" {}", width_description));
        }

        self.descriptors
            .iter()
            .filter(|descriptor| descriptor.is_pre())
            .for_each(|descriptor| parts.push(format!(" {}", descriptor)));

        parts.push(format!(" {}", &self.room_type));
        parts.push(".".to_string());

        self.descriptors
            .iter()
            .filter(|descriptor| !descriptor.is_pre())
            .for_each(|descriptor| parts.push(format!(" {}", descriptor.as_sentence())));

        parts.join("")
    }

    fn species_counts(&self) -> HashMap<Species, usize> {
        self.non_players
            .iter()
            .fold(HashMap::new(), |mut acc, npc| {
                let species = npc.character.species.clone();
                *acc.entry(species).or_insert(0) += 1;
                acc
            })
    }

    fn describe_inhabitants(&self) -> String {
        if self.non_players.is_empty() {
            return Self::empty_room_description();
        }

        let mut species_parts: Vec<String> = vec!["There".to_string()];
        for (index, (species, count)) in self.species_counts().iter().enumerate() {
            if index == 0 {
                if *count == 1 {
                    species_parts.push(" is ".to_string());
                } else {
                    species_parts.push(" are ".to_string());
                }
            } else {
                species_parts.push(", and ".to_string());
            }
            let species_description = Self::species_description(format!("{}", species), *count);
            species_parts.push(species_description);
        }

        format!("{}.", species_parts.join(""))
    }

    fn species_description(species_description: String, count: usize) -> String {
        if count == 1 {
            Self::single_species_description(species_description)
        } else if count == 2 {
            Self::two_species_description(species_description)
        } else {
            Self::multiple_species_description(species_description)
        }
    }

    fn two_species_description(species_description: String) -> String {
        let descriptions = vec![
            format!("a couple of {}s loitering about", species_description),
            format!("a couple of {}s standing around", species_description),
            format!("a couple of {}s", species_description),
            format!(
                "a couple of {}s glaring at you from nearby",
                species_description
            ),
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..descriptions.len());
        match descriptions.get(index) {
            Some(it) => it.clone(),
            _ => format!("a couple of {}s", species_description),
        }
    }

    fn multiple_species_description(species_description: String) -> String {
        let descriptions = vec![
            format!("a few {}s loitering around", species_description),
            format!("some {}s standing nearby", species_description),
            format!("some {}s", species_description),
            format!("a few {}s mulling about", species_description),
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..descriptions.len());
        match descriptions.get(index) {
            Some(it) => it.clone(),
            _ => format!("a few {}s", species_description),
        }
    }

    fn single_species_description(species_description: String) -> String {
        let descriptions = vec![
            format!("a lone {} glaring at you", species_description),
            format!("a single {}", species_description),
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..descriptions.len());
        match descriptions.get(index) {
            Some(it) => it.clone(),
            _ => format!("a lone {}", species_description),
        }
    }

    fn empty_room_description() -> String {
        let descriptions = vec![
            "There is no one around.",
            "It is empty.",
            "It is eerily empty.",
        ];

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..descriptions.len());
        match descriptions.get(index) {
            Some(it) => it.to_string(),
            _ => "The room is empty.".to_string(),
        }
    }
}
