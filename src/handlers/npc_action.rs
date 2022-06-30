use uuid::Uuid;

pub enum NpcAction {
    AttackPlayer(Uuid),
}
