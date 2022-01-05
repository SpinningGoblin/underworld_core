use super::{health::Health, dimensions::Dimensions};

#[derive(Clone, Debug)]
pub struct Stats {
    pub health: Option<Health>,
    pub dimensions: Option<Dimensions>
}
