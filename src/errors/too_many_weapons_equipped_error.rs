use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TooManyWeaponsEquippedError;

impl Display for TooManyWeaponsEquippedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyWeaponsEquipped")
    }
}

impl Error for TooManyWeaponsEquippedError {}
