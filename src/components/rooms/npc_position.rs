use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::{components::non_player::NonPlayer, utils::sentences::first_letter_to_upper_case};

use super::{group_descriptor::GroupDescriptor, npc_position_descriptor::NpcPositionDescriptor};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct NpcPosition {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub npcs: Vec<NonPlayer>,
    pub position_descriptor: Option<NpcPositionDescriptor>,
}

impl Display for NpcPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();

        for descriptor in self.position_descriptor.iter().filter(|d| d.is_pre()) {
            parts.push(format!("{}", descriptor));
        }

        if let Some(group_descriptor) = &self.group_descriptor {
            parts.push(format!("{}", group_descriptor));
        }

        parts.push(self.species_description());

        for descriptor in self.position_descriptor.iter().filter(|d| d.is_post()) {
            parts.push(format!("{}", descriptor));
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl NpcPosition {
    pub fn display_as_sentence(&self) -> String {
        first_letter_to_upper_case(format!("{}.", self))
    }

    fn species_description(&self) -> String {
        let species_counts = crate::utils::frequencies::sorted_frequencies(
            self.npcs.iter().map(|n| n.character.species.clone()),
        );

        let mut parts: Vec<String> = Vec::new();
        for (species, count) in species_counts {
            parts.push(species.describe_count(count));
        }

        parts.join(" and ")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{
            non_player::NonPlayer,
            rooms::{
                group_descriptor::GroupDescriptor, npc_position_descriptor::NpcPositionDescriptor,
            },
            species::Species,
        },
        generators::{
            characters::CharacterPrototype, generator::Generator, non_players::NonPlayerPrototype,
        },
    };

    use super::NpcPosition;

    #[test]
    fn display() {
        let goblin_prototype = NonPlayerPrototype {
            name: None,
            character_generator: Box::new(CharacterPrototype::overloaded_character(
                Species::Goblin,
            )),
        };
        let npcs: Vec<NonPlayer> = vec![goblin_prototype.generate(), goblin_prototype.generate()];

        let npc_position = NpcPosition {
            npcs,
            group_descriptor: Some(GroupDescriptor::AGangOf),
            position_descriptor: Some(NpcPositionDescriptor::InCornerStands),
        };

        assert_eq!(
            "in the corner stands a gang of goblins",
            format!("{}", &npc_position)
        );
    }
}
