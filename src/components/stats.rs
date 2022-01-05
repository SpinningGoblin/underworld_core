use super::{dimensions::Dimensions, health::Health};

#[derive(Clone, Debug)]
pub struct Stats {
    pub health: Option<Health>,
    pub dimensions: Option<Dimensions>,
}
