use crate::{
    actions::{
        action::Action,
        attack_npc::AttackNpc,
        exit_room::ExitRoom,
        inspect_npc::InspectNpc,
        look_at::LookAtCurrentRoom,
        look_at_npc::LookAtNpc,
        loot_npc::LootNpc,
    },
    components::{games::game_state::GameState, player::PlayerCharacter},
    errors::Errors,
    events::event::Event,
    handlers::{handle, HandledAction},
};

pub struct Game {
    pub state: GameState,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn handle_action(&mut self, action: &Action) -> Result<Vec<Event>, Errors> {
        let HandledAction {
            events,
            new_state,
            new_player,
        } = handle(action, &self.state, &self.player)?;
        self.state = new_state;
        self.player = new_player;

        Ok(events)
    }

    pub fn current_actions(&self) -> Vec<Action> {
        let room_view_actions = vec![Action::LookAtCurrentRoom(LookAtCurrentRoom)];

        let npc_actions = self
            .state
            .current_room()
            .npc_positions
            .iter()
            .flat_map(|npc_position| npc_position.npcs.iter())
            .flat_map(|npc| {
                let mut actions = vec![
                    Action::LookAtNpc(LookAtNpc {
                        npc_id: npc.identifier.id.to_string(),
                    }),
                    Action::InspectNpc(InspectNpc {
                        npc_id: npc.identifier.id.to_string(),
                        discover_health: true,
                        discover_name: true,
                        discover_packed_items: true,
                        discover_hidden_items: true,
                    }),
                ];

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
            });

        let exit_actions = self.state.current_room_exits().into_iter().map(|id| {
            Action::ExitRoom(ExitRoom {
                exit_id: id.to_string(),
            })
        });

        room_view_actions
            .into_iter()
            .chain(npc_actions)
            .chain(exit_actions)
            .collect()
    }
}
