use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ItemNotFoundError(pub String);

impl Display for ItemNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemNotFound:{}", self.0)
    }
}

impl Error for ItemNotFoundError {}
