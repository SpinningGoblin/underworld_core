#[derive(Debug, thiserror::Error, strum_macros::Display)]
pub enum Error {
    ExitNotFoundError(String),
    FixtureNotFoundError(String),
    InvalidIdError(String),
    ItemNotDirectlyUsableError(String),
    ItemNotFoundError(String),
    NpcNotFoundError(String),
    PlayerIsDeadError,
    SpellNotFoundError(String),
    TooManyWeaponsEquippedError,
    TooManyWearablesEquippedError,
}
