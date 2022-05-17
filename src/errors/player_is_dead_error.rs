use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PlayerIsDeadError;

impl Display for PlayerIsDeadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerIsDead")
    }
}

impl Error for PlayerIsDeadError {}
