use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct InvalidIdError(pub String);

impl Display for InvalidIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidId:{}", self.0)
    }
}
