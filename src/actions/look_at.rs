use std::str::FromStr;

use uuid::Uuid;

pub struct LookAtTarget {
    pub target: String,
}

impl LookAtTarget {
    pub fn target_id(&self) -> Option<Uuid> {
        self.id(&self.target)
    }

    pub fn description(&self) -> String {
        "Look at a target inside of a room".to_string()
    }

    fn id(&self, value: &str) -> Option<Uuid> {
        Uuid::from_str(value).map_or(None, |id| Some(id))
    }
}

pub struct LookAtRoom;

impl LookAtRoom {
    pub fn description(&self) -> String {
        "Look at a room".to_string()
    }
}
