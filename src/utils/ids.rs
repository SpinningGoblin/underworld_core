use std::{error::Error, str::FromStr};

use uuid::Uuid;

use crate::errors::invalid_id_error::InvalidIdError;

pub fn parse_id(id: &str) -> Result<Uuid, Box<dyn Error>> {
    match Uuid::from_str(id) {
        Ok(it) => Ok(it),
        Err(_) => Err(Box::new(InvalidIdError(id.to_string()))),
    }
}
