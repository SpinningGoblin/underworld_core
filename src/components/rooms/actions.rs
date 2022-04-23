use crate::actions::{
    action::Action,
    attack_npc::AttackNpc,
    exit_room::ExitRoom,
    look_at::{LookAtRoom, LookAtTarget},
    quick_look::QuickLookRoom,
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
                let mut actions = vec![Action::LookAtTarget(LookAtTarget {
                    target: npc.identifier.id.to_string(),
                    room_id: self.identifier.id.to_string(),
                })];

                if !npc.character.is_dead() {
                    actions.push(Action::AttackNpc(AttackNpc {
                        target_id: npc.identifier.id.to_string(),
                    }))
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
