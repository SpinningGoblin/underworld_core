use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidIdError(pub String);

impl Display for InvalidIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidId:{}", self.0)
    }
}

impl Error for InvalidIdError {}
