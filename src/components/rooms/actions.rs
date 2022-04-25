use crate::{
    actions::{
        action::Action,
        attack_npc::AttackNpc,
        exit_room::ExitRoom,
        look_at::{LookAtNpc, LookAtRoom, LookAtTarget},
        loot_npc::LootNpc,
        quick_look::QuickLookRoom,
    },
    components::character::CharacterViewArgs,
};

use super::room::Room;

impl Room {
    pub fn current_actions(&self) -> Vec<Action> {
        let basic_actions = vec![
            Action::QuickLookRoom(QuickLookRoom {
                room_id: self.identifier.id.to_string(),
            }),
            Action::LookAtRoom(LookAtRoom {
                room_id: self.identifier.id.to_string(),
            }),
        ];

        let fixture_actions = self.fixture_positions.iter().flat_map(|fixture_position| {
            fixture_position.fixtures.iter().map(|fixture| {
                Action::LookAtTarget(LookAtTarget {
                    target: fixture.identifier.id.to_string(),
                    room_id: self.identifier.id.to_string(),
                })
            })
        });

        let npc_actions = self.npc_positions.iter().flat_map(|npc_position| {
            npc_position.npcs.iter().flat_map(|npc| {
                let args = CharacterViewArgs {
                    knows_health: true,
                    knows_species: true,
                    knows_life_modifier: true,
                    knows_inventory: true,
                    knows_hidden_in_inventory: false,
                    knows_packed_in_inventory: false,
                };

                let mut actions = vec![Action::LookAtNpc(LookAtNpc {
                    npc_id: npc.identifier.id.to_string(),
                    knows_all: false,
                    knows_name: true,
                    args,
                })];

                if !npc.character.is_dead() {
                    actions.push(Action::AttackNpc(AttackNpc {
                        npc_id: npc.identifier.id.to_string(),
                    }));
                } else {
                    let item_ids = match &npc.character.inventory {
                        Some(it) => it
                            .equipment
                            .iter()
                            .map(|character_item| character_item.item.identifier.id.to_string())
                            .collect(),
                        None => Vec::new(),
                    };

                    actions.push(Action::LootNpc(LootNpc {
                        npc_id: npc.identifier.id.to_string(),
                        item_ids,
                    }));
                }

                actions
            })
        });

        let exit_actions = self.exits.iter().map(|exit| {
            Action::ExitRoom(ExitRoom {
                exit_id: exit.identifier.id.to_string(),
            })
        });

        basic_actions
            .into_iter()
            .chain(fixture_actions)
            .chain(npc_actions)
            .chain(exit_actions)
            .collect()
    }
}
