use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FixtureNotFoundError(pub String);

impl Display for FixtureNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixtureNotFound:{}", self.0)
    }
}

impl Error for FixtureNotFoundError {}
