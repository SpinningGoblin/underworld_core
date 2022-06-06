use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct ExitNotFoundError(pub String);

impl Display for ExitNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExitNotFound:{}", self.0)
    }
}
