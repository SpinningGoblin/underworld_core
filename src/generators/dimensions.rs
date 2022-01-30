use std::ops::Range;

use rand::Rng;

use crate::components::{dimensions::Dimensions, rooms::room_type::RoomType};

use super::generator::Generator;

pub struct DimensionsPrototype {
    pub height_range: Range<f32>,
    pub width_range: Range<f32>,
}

impl Generator<Dimensions> for DimensionsPrototype {
    fn generate(&self) -> Dimensions {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(self.width_range.clone());
        let height = rng.gen_range(self.height_range.clone());

        Dimensions { height, width }
    }
}

impl DimensionsPrototype {
    pub fn for_room_type(room_type: &RoomType) -> Self {
        match *room_type {
            RoomType::Cave => Self {
                height_range: 4.2..5.8,
                width_range: 4.3..5.8,
            },
            RoomType::Cavern => Self {
                height_range: 8.0..13.0,
                width_range: 8.0..13.0,
            },
            RoomType::PrisonCell => Self {
                height_range: 1.3..2.0,
                width_range: 1.3..2.0,
            },
            RoomType::Room => Self {
                height_range: 2.0..4.0,
                width_range: 2.0..4.0,
            },
            RoomType::EntryWay => Self {
                height_range: 1.3..2.0,
                width_range: 1.3..2.0,
            },
        }
    }
}
