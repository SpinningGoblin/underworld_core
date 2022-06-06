pub mod error;
pub mod exit_not_found_error;
pub mod fixture_not_found_error;
pub mod invalid_id_error;
pub mod item_not_directly_usable;
pub mod item_not_found_error;
pub mod npc_not_found_error;
pub mod player_is_dead_error;
pub mod spell_not_found_error;
pub mod too_many_weapons_equipped_error;

pub use {
    error::Error, exit_not_found_error::ExitNotFoundError,
    fixture_not_found_error::FixtureNotFoundError, invalid_id_error::InvalidIdError,
    item_not_directly_usable::ItemNotDirectlyUsableError, item_not_found_error::ItemNotFoundError,
    npc_not_found_error::NpcNotFoundError, player_is_dead_error::PlayerIsDeadError,
    spell_not_found_error::SpellNotFoundError,
    too_many_weapons_equipped_error::TooManyWeaponsEquippedError,
};
