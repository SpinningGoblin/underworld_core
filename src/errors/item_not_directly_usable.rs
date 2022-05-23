use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ItemNotDirectlyUsableError(pub String);

impl Display for ItemNotDirectlyUsableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemNotDirectlyUsable:{}", self.0)
    }
}

impl Error for ItemNotDirectlyUsableError {}
