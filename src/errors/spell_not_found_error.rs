use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct SpellNotFoundError(pub String);

impl Display for SpellNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpellNotFound:{}", self.0)
    }
}

impl Error for SpellNotFoundError {}
