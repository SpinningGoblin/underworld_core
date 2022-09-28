pub mod ids;
pub mod rolls;

pub use ids::parse_id;
pub use rolls::{roll_d100, roll_d6, roll_percent_succeeds};
