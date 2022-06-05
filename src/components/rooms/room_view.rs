use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use rand::Rng;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{non_player::NonPlayerView, species::Species};

use super::{
    descriptor::Descriptor, dimensions::Dimensions, exit::ExitView,
    fixture_position::FixturePositionView, flavour::Flavour, npc_position::NpcPositionView,
    room_type::RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Room"))]
pub struct RoomView {
    pub id: String,
    pub name: Option<String>,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub fixture_positions: Vec<FixturePositionView>,
    pub dimensions: Dimensions,
    pub npc_positions: Vec<NpcPositionView>,
    pub flavour: Option<Flavour>,
    pub exits: Vec<ExitView>,
}

#[derive(Clone, Debug, Default)]
pub struct RoomViewArgs {
    pub can_see_hidden: bool,
    pub can_see_packed: bool,
    pub knows_character_health: bool,
    pub knows_fixture_items: bool,
    pub knows_fixture_hidden: bool,
    pub knows_fixture_can_be_opened: bool,
    pub knows_fixture_has_hidden: bool,
}

impl Display for RoomView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![
            self.display_room(),
            self.display_fixtures(),
            self.display_inhabitants(),
        ];

        write!(f, "{}", parts.join(" "))
    }
}

impl RoomView {
    pub fn describe_inhabitants(&self) -> String {
        let non_players: Vec<&NonPlayerView> = self
            .npc_positions
            .iter()
            .map(|npc_position| &npc_position.npc)
            .collect();
        if non_players.is_empty() {
            return "".to_string();
        }

        if non_players.len() == 1 {
            let npc = non_players.get(0).unwrap();
            return npc.describe("");
        }

        let mut descriptions: Vec<String> = Vec::new();
        let original_species_counts = self.species_counts(&non_players);
        let mut current_counts = original_species_counts.clone();
        for npc in non_players.iter() {
            if !npc.character.species_known {
                continue;
            }

            let (original_species_count, current_species_count) = match &npc.character.species {
                Some(it) => (
                    original_species_counts.get(it).unwrap(),
                    current_counts.get(it).unwrap(),
                ),
                None => continue,
            };

            if original_species_count.eq(&1) {
                descriptions.push(npc.describe(""));
            } else {
                let starter = get_count_text(original_species_count, current_species_count);
                descriptions.push(npc.describe(&starter));
                let new_count = *current_species_count - 1;
                if let Some(species) = &npc.character.species {
                    current_counts.insert(species.clone(), new_count).unwrap();
                }
            }
        }

        descriptions.join(" ")
    }

    fn display_room(&self) -> String {
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

    fn species_counts(&self, non_players: &[&NonPlayerView]) -> HashMap<Species, usize> {
        non_players.iter().fold(HashMap::new(), |mut acc, npc| {
            if npc.character.species_known {
                if let Some(species) = &npc.character.species {
                    *acc.entry(species.clone()).or_insert(0) += 1;
                }
            }
            acc
        })
    }

    fn display_fixtures(&self) -> String {
        if self.fixture_positions.is_empty() {
            return "".to_string();
        }

        if self.fixture_positions.is_empty() {
            return "".to_string();
        }

        let all_fixtures: Vec<String> = self
            .fixture_positions
            .iter()
            .map(|fixture_position| fixture_position.display_as_sentence())
            .collect();
        all_fixtures.join(" ")
    }

    fn display_inhabitants(&self) -> String {
        if self.npc_positions.is_empty() {
            return Self::empty_room_description();
        }

        if self.npc_positions.is_empty() {
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
