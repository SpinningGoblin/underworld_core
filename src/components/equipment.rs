use self::location_tag::LocationTag;

pub mod location_descriptor;
pub mod location_tag;

pub trait Equipment {
    fn character_location_tags(&self) -> Vec<LocationTag>;
    fn possible_location_tags(&self) -> Vec<LocationTag>;
}
