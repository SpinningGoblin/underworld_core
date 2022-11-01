use std::collections::HashMap;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    components::{
        fixtures::FixtureViewArgs,
        rooms::{Room, RoomView},
        worlds::{World, WorldView},
        CharacterViewArgs, Ghost, NonPlayerViewArgs,
    },
    systems::view::room::view,
};

use super::{CharacterKnowledge, FixtureKnowledge, Statistics};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct GameState {
    pub id: Uuid,
    pub name: Option<String>,
    pub world: World,
    pub current_room_id: Uuid,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub rooms_seen: Vec<Uuid>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub all_knowledge_unlocked: bool,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub player_npc_knowledge: HashMap<Uuid, CharacterKnowledge>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub player_fixture_knowledge: HashMap<Uuid, FixtureKnowledge>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub player_statistics: HashMap<Uuid, Statistics>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub danger_level: u32,
    #[cfg_attr(feature = "serialization", serde(default, skip))]
    pub ghosts: Vec<Ghost>,
}

impl GameState {
    pub fn get_player_stats(&self, player_id: &Uuid) -> Option<Statistics> {
        self.player_statistics.get(player_id).cloned()
    }

    pub fn npc_knowledge(&self, npc_id: &Uuid) -> CharacterKnowledge {
        self.player_npc_knowledge
            .get(npc_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn fixture_knowledge(&self, fixture_id: &Uuid) -> FixtureKnowledge {
        self.player_fixture_knowledge
            .get(fixture_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_npc_knowledge(&mut self, npc_id: Uuid, knowledge: CharacterKnowledge) {
        self.player_npc_knowledge.insert(npc_id, knowledge);
    }

    pub fn add_player_kill_to_stats(&mut self, pc_id: &Uuid) {
        let statistics = self.player_statistics.entry(*pc_id).or_default();
        statistics.num_killed += 1;
    }

    pub fn add_player_damage_taken_to_stats(&mut self, pc_id: &Uuid, damage: i32) {
        let statistics = self.player_statistics.entry(*pc_id).or_default();
        statistics.total_damage_taken += damage;
    }

    pub fn add_player_damage_done_to_stats(&mut self, pc_id: &Uuid, damage: i32) {
        let statistics = self.player_statistics.entry(*pc_id).or_default();
        statistics.total_damage_done += damage;
    }

    pub fn player_stats(&self, pc_id: &Uuid) -> Statistics {
        self.player_statistics
            .get(pc_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_fixture_knowledge(&mut self, fixture_id: Uuid, knowledge: FixtureKnowledge) {
        self.player_fixture_knowledge.insert(fixture_id, knowledge);
    }

    pub fn current_room_exits(&self) -> Vec<Uuid> {
        self.current_room()
            .exits
            .iter()
            .map(|exit| exit.id)
            .collect()
    }

    pub fn current_room(&self) -> &Room {
        self.world
            .rooms
            .iter()
            .find(|room| room.id.eq(&self.current_room_id))
            .unwrap()
    }

    pub fn current_room_mut(&mut self) -> &mut Room {
        self.world
            .rooms
            .iter_mut()
            .find(|room| room.id.eq(&self.current_room_id))
            .unwrap()
    }

    pub fn view_room(&self, room: &Room) -> RoomView {
        let mut fixture_args: HashMap<Uuid, FixtureViewArgs> = HashMap::new();

        for fixture_id in room
            .fixture_positions
            .iter()
            .map(|fixture_position| fixture_position.fixture.id)
        {
            let knowledge = self.fixture_knowledge(&fixture_id);

            fixture_args.insert(
                fixture_id,
                FixtureViewArgs {
                    knows_has_hidden_compartment: knowledge.knows_has_hidden_compartment,
                },
            );
        }

        let mut npc_args: HashMap<Uuid, NonPlayerViewArgs> = HashMap::new();

        for npc_id in room
            .npc_positions
            .iter()
            .map(|npc_position| &npc_position.npc)
            .map(|npc| npc.id)
        {
            let knowledge = self.npc_knowledge(&npc_id);
            npc_args.insert(
                npc_id,
                NonPlayerViewArgs {
                    character_args: CharacterViewArgs {
                        knows_health: knowledge.knows_health,
                        knows_inventory: knowledge.knows_inventory,
                        knows_packed_in_inventory: knowledge.knows_packed_in_inventory,
                    },
                },
            );
        }

        let mut exit_visitations: HashMap<Uuid, bool> = HashMap::new();
        let room_id = room.id;
        for exit in room.exits.iter() {
            let exit_map = match self
                .world
                .exit_graph
                .iter()
                .find(|exit_map| exit_map.exit_id.eq(&exit.id))
            {
                Some(it) => it,
                None => continue,
            };

            let has_visited_other_room = exit_map.other_room_id(room_id).is_some();

            exit_visitations.insert(exit.id, has_visited_other_room);
        }

        view(
            room,
            npc_args,
            fixture_args,
            exit_visitations,
            self.all_knowledge_unlocked,
        )
    }

    pub fn view_current_room(&self) -> RoomView {
        self.view_room(self.current_room())
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "GameState"))]
pub struct GameStateView {
    pub id: String,
    pub name: Option<String>,
    pub world: WorldView,
    pub current_room_id: String,
    pub rooms_seen: Vec<String>,
    pub all_knowledge_unlocked: bool,
    pub player_npc_knowledge: HashMap<String, CharacterKnowledge>,
    pub player_fixture_knowledge: HashMap<String, FixtureKnowledge>,
    pub player_statistics: HashMap<String, Statistics>,
    pub danger_level: u32,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::generators::{game::game_generator, generator::Generator};

    #[test]
    fn stats_can_be_incremented() {
        let mut state = game_generator().generate();
        let pc_id = Uuid::new_v4();
        state.add_player_kill_to_stats(&pc_id);
        let stats = state.player_stats(&pc_id);

        assert_eq!(stats.num_killed, 1);
    }

    #[test]
    #[cfg(feature = "serialization")]
    fn can_be_deserialized_without_ghosts() {
        use std::fs;

        use super::GameState;

        let text = fs::read_to_string("./fixtures/game.json").unwrap();
        let game_state: GameState = serde_json::from_str(&text).unwrap();

        assert_eq!(game_state.ghosts.len(), 0);
    }

    #[test]
    #[cfg(feature = "serialization")]
    fn serialization_skips_ghosts() {
        use std::fs;

        use super::GameState;
        use crate::components::{
            spells::SpellMemory, Character, Effects, Ghost, Health, Inventory, Stats,
        };

        let text = fs::read_to_string("./fixtures/game.json").unwrap();
        let mut game_state: GameState = serde_json::from_str(&text).unwrap();

        game_state.ghosts.push(Ghost {
            character: Character {
                stats: Stats {
                    health: Health::from_max(2),
                    height: crate::components::Size::Average,
                    base_attack: None,
                    base_damage_resistance: None,
                },
                species: crate::components::Species::Bugbear,
                life_modifier: None,
                inventory: Inventory::default(),
                current_effects: Effects::default(),
                spell_memory: SpellMemory::default(),
            },
            name: None,
        });

        let serialized = serde_json::to_string(&game_state).unwrap();

        assert!(!serialized.contains("\"ghosts\":"));
    }
}
