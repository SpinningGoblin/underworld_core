use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct NpcNotFoundError(pub String);

impl Display for NpcNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NpcNotFound:{}", self.0)
    }
}
