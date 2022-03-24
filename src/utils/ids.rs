use std::str::FromStr;

use uuid::Uuid;

pub fn parse_id(id: &str) -> Option<Uuid> {
    Uuid::from_str(id).ok()
}
