#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use poem_openapi::Object;

use crate::components::{character::CharacterViewArgs, identifier::IdentifierView};

use super::{
    descriptor::Descriptor,
    dimensions::Dimensions,
    fixture_position::FixturePositionView,
    flavour::Flavour,
    npc_position::{NpcPositionView, NpcPositionViewArgs},
    room::Room,
    room_type::RoomType,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct RoomView {
    pub identifier: IdentifierView,
    pub descriptors: Vec<Descriptor>,
    pub room_type: RoomType,
    pub fixture_positions: Vec<FixturePositionView>,
    pub dimensions: Dimensions,
    pub npc_positions: Vec<NpcPositionView>,
    pub flavour: Option<Flavour>,
}

#[derive(Clone, Debug)]
pub struct RoomViewArgs {
    pub can_see_hidden: bool,
    pub can_see_packed: bool,
    pub knows_character_health: bool,
    pub knows_names: bool,
}

impl Room {
    pub fn quick_look(&self) -> RoomView {
        let npc_position_args = NpcPositionViewArgs {
            character_args: CharacterViewArgs {
                knows_health: false,
                knows_species: true,
                knows_life_modifier: true,
                knows_inventory: false,
                knows_hidden_in_inventory: false,
                knows_packed_in_inventory: false,
            },
            knows_name: false,
        };

        self.room_view(npc_position_args, false)
    }

    pub fn look_at(&self, args: RoomViewArgs, knows_all: bool) -> RoomView {
        let npc_position_args = NpcPositionViewArgs {
            character_args: CharacterViewArgs {
                knows_health: args.knows_character_health,
                knows_species: true,
                knows_life_modifier: true,
                knows_inventory: true,
                knows_hidden_in_inventory: args.can_see_hidden,
                knows_packed_in_inventory: args.can_see_packed,
            },
            knows_name: args.knows_names,
        };

        self.room_view(npc_position_args, knows_all)
    }

    fn room_view(&self, npc_position_args: NpcPositionViewArgs, knows_all: bool) -> RoomView {
        let fixture_positions: Vec<FixturePositionView> = self
            .fixture_positions
            .iter()
            .map(|fixture_position| fixture_position.look_at())
            .into_iter()
            .collect();
        let npc_positions: Vec<NpcPositionView> = self
            .npc_positions
            .iter()
            .map(|npc_position| npc_position.look_at(&npc_position_args, knows_all))
            .into_iter()
            .collect();

        RoomView {
            identifier: IdentifierView {
                id: self.identifier.id.to_string(),
                name: self.identifier.name.clone(),
                name_known: true,
            },
            descriptors: self.descriptors.clone(),
            room_type: self.room_type.clone(),
            fixture_positions,
            dimensions: self.dimensions.clone(),
            npc_positions,
            flavour: self.flavour.clone(),
        }
    }
}
