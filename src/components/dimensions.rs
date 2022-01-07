use std::ops::Range;

use super::species::Species;

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub height: f32,
    pub width: f32,
}

const TALL: &str = "tall";
const SHORT: &str = "short";
const AVERAGE_HEIGHT: &str = "";

impl Dimensions {
    pub fn describe_height_for_species(&self, species: &Species) -> String {
        self.calculate_height(&species.height_range())
    }

    fn calculate_height(&self, height_range: &Range<f32>) -> String {
        if self.is_taller_than(height_range.end) {
            TALL.to_string()
        } else if self.is_shorter_than(height_range.start) {
            SHORT.to_string()
        } else {
            AVERAGE_HEIGHT.to_string()
        }
    }

    fn is_taller_than(&self, height: f32) -> bool {
        self.height > height
    }

    fn is_shorter_than(&self, height: f32) -> bool {
        self.height < height
    }
}

#[cfg(test)]
mod test {
    use crate::components::species::Species;

    use super::Dimensions;

    #[test]
    fn describe_height_for_species_when_taller() {
        let dimensions = Dimensions {
            height: 1.5,
            width: 0.1,
        };
        assert_eq!(
            "tall",
            dimensions.describe_height_for_species(&Species::Goblin)
        );
    }

    #[test]
    fn describe_height_for_species_when_shorter() {
        let dimensions = Dimensions {
            height: 0.4,
            width: 0.1,
        };
        assert_eq!(
            "short",
            dimensions.describe_height_for_species(&Species::Goblin)
        );
    }

    #[test]
    fn describe_height_for_species_when_average() {
        let dimensions = Dimensions {
            height: 0.7,
            width: 0.1,
        };
        assert_eq!("", dimensions.describe_height_for_species(&Species::Goblin));
    }
}
