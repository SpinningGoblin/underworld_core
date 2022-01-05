use super::{stats::Stats, name::Name};

#[derive(Clone, Debug)]
pub struct Character {
    pub name: Option<Name>,
    pub stats: Stats,
}
