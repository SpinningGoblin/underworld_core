use super::health::Health;

#[derive(Clone, Debug)]
pub struct Stats {
    pub health: Option<Health>,
}
