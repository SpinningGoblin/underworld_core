use super::{
    ExitNotFoundError, FixtureNotFoundError, InvalidIdError, ItemNotDirectlyUsableError,
    ItemNotFoundError, NpcNotFoundError, PlayerIsDeadError, SpellNotFoundError,
    TooManyWeaponsEquippedError,
};

#[derive(Debug)]
pub enum Error {
    ExitNotFoundError(ExitNotFoundError),
    FixtureNotFoundError(FixtureNotFoundError),
    InvalidIdError(InvalidIdError),
    ItemNotDirectlyUsableError(ItemNotDirectlyUsableError),
    ItemNotFoundError(ItemNotFoundError),
    NpcNotFoundError(NpcNotFoundError),
    PlayerIsDeadError(PlayerIsDeadError),
    SpellNotFoundError(SpellNotFoundError),
    TooManyWeaponsEquippedError(TooManyWeaponsEquippedError),
}
