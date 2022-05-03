use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Errors {
    ExitNotFound(String),
    ItemNotFound(String),
    InvalidId(String),
    NpcNotFound(String),
    TooManyWeaponsEquipped,
}
