use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ExitNotFoundError(pub String);

impl Display for ExitNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExitNotFound:{}", self.0)
    }
}

impl Error for ExitNotFoundError {}
