use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct NpcNotFoundError(pub String);

impl Display for NpcNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NpcNotFound:{}", self.0)
    }
}

impl Error for NpcNotFoundError {}
