use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct PlayerIsDeadError;

impl Display for PlayerIsDeadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerIsDead")
    }
}
