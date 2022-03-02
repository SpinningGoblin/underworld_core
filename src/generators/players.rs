use crate::components::{size::Size, species::Species};

pub struct PlayerCharacterPrototype {
    pub username: String,
    pub species: Option<Species>,
    pub size: Option<Size>,
}
