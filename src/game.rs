use std::error::Error;

use crate::{
    actions::{
        action::Action, attack_npc::AttackNpc, exit_room::ExitRoom,
        inspect_fixture::InspectFixture, inspect_npc::InspectNpc,
        look_at_current_room::LookAtCurrentRoom, look_at_fixture::LookAtFixture,
        look_at_npc::LookAtNpc, loot_npc::LootNpc, CastSpellOnNpc, CastSpellOnPlayer, LootFixture,
        MovePlayerItem, UseItemOnPlayer,
    },
    components::{
        games::game_state::GameState,
        items::{
            consumable_effect::ConsumableEffectName,
            location_tag::{packed_tags_for_item_type, ready_tag_for_item_type},
        },
        player::PlayerCharacter,
    },
    events::event::Event,
    handlers::handle::HandledAction,
};

pub struct Game {
    pub state: GameState,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn handle_action(&mut self, action: &Action) -> Result<Vec<Event>, Box<dyn Error>> {
        let HandledAction {
            events,
            new_state,
            new_player,
        } = crate::handlers::handle::handle_action(action, &self.state, &self.player)?;
        self.state = new_state;
        self.player = new_player;

        Ok(events)
    }

    pub fn current_actions(&self) -> Vec<Action> {
        let room_view_actions = vec![Action::LookAtCurrentRoom(LookAtCurrentRoom)];

        let fixture_actions = self
            .state
            .current_room()
            .fixture_positions
            .iter()
            .flat_map(|fixture_position| Some(&fixture_position.fixture))
            .flat_map(|fixture| {
                let mut actions = vec![
                    Action::LookAtFixture(LookAtFixture {
                        fixture_id: fixture.id.to_string(),
                    }),
                    Action::InspectFixture(InspectFixture {
                        fixture_id: fixture.id.to_string(),
                        discover_hidden: true,
                        discover_hidden_items: true,
                        discover_contained: true,
                        discover_can_be_opened: true,
                    }),
                ];

                let knowledge = self.state.fixture_knowledge(&fixture.id);
                let mut item_ids: Vec<String> = Vec::new();

                if knowledge.knows_items {
                    for fixture_item in fixture
                        .items
                        .iter()
                        .filter(|fixture_item| !fixture_item.is_hidden)
                    {
                        item_ids.push(fixture_item.item.id.to_string());
                    }
                }

                if knowledge.knows_hidden_items {
                    for fixture_item in fixture
                        .items
                        .iter()
                        .filter(|fixture_item| fixture_item.is_hidden)
                    {
                        item_ids.push(fixture_item.item.id.to_string());
                    }
                }

                if !item_ids.is_empty() {
                    actions.push(Action::LootFixture(LootFixture {
                        fixture_id: fixture.id.to_string(),
                        item_ids,
                    }))
                }

                actions
            });

        let npc_actions = self
            .state
            .current_room()
            .npc_positions
            .iter()
            .map(|npc_position| &npc_position.npc)
            .flat_map(|npc| {
                let mut actions = vec![
                    Action::LookAtNpc(LookAtNpc {
                        npc_id: npc.id.to_string(),
                    }),
                    Action::InspectNpc(InspectNpc {
                        npc_id: npc.id.to_string(),
                        discover_health: true,
                        discover_name: true,
                        discover_packed_items: true,
                        discover_hidden_items: true,
                    }),
                ];

                if !npc.character.is_dead() {
                    actions.push(Action::AttackNpc(AttackNpc {
                        npc_id: npc.id.to_string(),
                    }));

                    for learned_spell in self.player.character.spell_memory.spells.iter() {
                        actions.push(Action::CastSpellOnNpc(CastSpellOnNpc {
                            spell_id: learned_spell.id.to_string(),
                            npc_id: npc.id.to_string(),
                        }));
                    }
                } else {
                    let item_ids = npc
                        .character
                        .inventory
                        .equipment
                        .iter()
                        .map(|character_item| character_item.item.id.to_string())
                        .collect();

                    actions.push(Action::LootNpc(LootNpc {
                        npc_id: npc.id.to_string(),
                        item_ids,
                    }));
                }

                actions
            });

        let exit_actions = self.state.current_room_exits().into_iter().map(|id| {
            Action::ExitRoom(ExitRoom {
                exit_id: id.to_string(),
            })
        });

        let spell_actions = self
            .player
            .character
            .spell_memory
            .spells
            .iter()
            .map(|learned_spell| {
                Action::CastSpellOnPlayer(CastSpellOnPlayer {
                    spell_id: learned_spell.id.to_string(),
                })
            });

        let item_actions =
            self.player
                .character
                .inventory
                .equipment
                .iter()
                .flat_map(|character_item| {
                    let mut actions: Vec<Action> = Vec::new();

                    if character_item.is_consumable() {
                        match &character_item.item.consumable {
                            Some(consumable) => match &consumable.effect.name {
                                ConsumableEffectName::LearnSpell => {
                                    actions.push(Action::UseItemOnPlayer(UseItemOnPlayer {
                                        item_id: character_item.item.id.to_string(),
                                    }));
                                }
                            },
                            None => {}
                        }
                    } else if character_item.is_packed() {
                        let location_tag = ready_tag_for_item_type(&character_item.item.item_type);
                        actions.push(Action::MovePlayerItem(MovePlayerItem {
                            item_id: character_item.item.id.to_string(),
                            location_tag,
                            put_at_the_ready: true,
                        }));
                    } else {
                        for location_tag in
                            packed_tags_for_item_type(&character_item.item.item_type).into_iter()
                        {
                            actions.push(Action::MovePlayerItem(MovePlayerItem {
                                item_id: character_item.item.id.to_string(),
                                location_tag,
                                put_at_the_ready: false,
                            }));
                        }
                    }

                    actions
                });

        room_view_actions
            .into_iter()
            .chain(npc_actions)
            .chain(exit_actions)
            .chain(fixture_actions)
            .chain(spell_actions)
            .chain(item_actions)
            .collect()
    }
}
