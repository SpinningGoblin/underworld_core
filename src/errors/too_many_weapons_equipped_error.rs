use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub struct TooManyWeaponsEquippedError;

impl Display for TooManyWeaponsEquippedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyWeaponsEquipped")
    }
}
