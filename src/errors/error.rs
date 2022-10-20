#[derive(Clone, Debug, thiserror::Error, strum_macros::Display)]
pub enum Error {
    ExitNotFoundError(String),
    FixtureNotFoundError(String),
    FixtureCannotBeFound(String),
    FixtureCannotBeOpened(String),
    FixtureHasNoHiddenCompartment(String),
    FixtureHasHiddenCompartmentUnknown(String),
    InvalidIdError(String),
    ItemNotDirectlyUsableError(String),
    ItemNotFoundError(String),
    ItemNotThrowableError(String),
    NpcNotFoundError(String),
    PlayerIsDeadError,
    SpellNotFoundError(String),
    TooManyWeaponsEquippedError,
    TooManyWearablesEquippedError,
    ItemCannotBeTakenFromFixture(String),
}
