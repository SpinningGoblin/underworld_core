use std::str::FromStr;

use uuid::Uuid;

use crate::errors::Errors;

pub fn parse_id(id: &str) -> Result<Uuid, Errors> {
    match Uuid::from_str(id) {
        Ok(it) => Ok(it),
        Err(_) => Err(Errors::InvalidId(id.to_string())),
    }
}
