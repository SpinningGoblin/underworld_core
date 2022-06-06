use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct FixtureNotFoundError(pub String);

impl Display for FixtureNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixtureNotFound:{}", self.0)
    }
}
