#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::dimension_descriptors::{HeightDescriptor, WidthDescriptor};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Dimensions {
    pub height: f32,
    pub width: f32,
}

impl Dimensions {
    pub fn is_average_height(&self, descriptor: &impl HeightDescriptor) -> bool {
        let height_range = descriptor.height_range();

        !self.is_larger_than(height_range.end) && !self.is_smaller_than(height_range.start)
    }

    pub fn describe_height(&self, descriptor: &impl HeightDescriptor) -> String {
        let height_range = descriptor.height_range();
        if self.is_larger_than(height_range.end) {
            descriptor.bigger_text()
        } else if self.is_smaller_than(height_range.start) {
            descriptor.smaller_text()
        } else {
            descriptor.average_text()
        }
    }

    pub fn describe_width(&self, descriptor: &impl WidthDescriptor) -> String {
        let width_range = descriptor.width_range();
        if self.is_larger_than(width_range.end) {
            descriptor.bigger_text()
        } else if self.is_smaller_than(width_range.start) {
            descriptor.smaller_text()
        } else {
            descriptor.average_text()
        }
    }

    fn is_larger_than(&self, height: f32) -> bool {
        self.height > height
    }

    fn is_smaller_than(&self, height: f32) -> bool {
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
        assert_eq!("tall", dimensions.describe_height(&Species::Goblin));
    }

    #[test]
    fn describe_height_for_species_when_shorter() {
        let dimensions = Dimensions {
            height: 0.4,
            width: 0.1,
        };
        assert_eq!("short", dimensions.describe_height(&Species::Goblin));
    }

    #[test]
    fn describe_height_for_species_when_average() {
        let dimensions = Dimensions {
            height: 0.7,
            width: 0.1,
        };
        assert_eq!("", dimensions.describe_height(&Species::Goblin));
    }
}
