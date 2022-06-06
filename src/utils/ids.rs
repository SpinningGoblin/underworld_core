use std::str::FromStr;

use uuid::Uuid;

use crate::errors::{Error, InvalidIdError};

pub fn parse_id(id: &str) -> Result<Uuid, Error> {
    match Uuid::from_str(id) {
        Ok(it) => Ok(it),
        Err(_) => Err(Error::InvalidIdError(InvalidIdError(id.to_string()))),
    }
}
