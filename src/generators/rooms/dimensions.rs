use rand::Rng;

use crate::{
    components::{rooms::Dimensions, Size},
    utils::rolls::roll_percent_succeeds,
};

const NON_AVERAGE_HEIGHT_CHANGE: i32 = 25;
const NON_AVERAGE_WIDTH_CHANGE: i32 = 50;
const NON_AVERAGE_LENGTH_CHANGE: i32 = 25;

fn non_average_heights() -> Vec<Size> {
    vec![Size::Tall, Size::Squat]
}

fn non_average_widths() -> Vec<Size> {
    vec![Size::Huge, Size::Massive, Size::Narrow, Size::Wide]
}

fn non_average_lengths() -> Vec<Size> {
    vec![Size::Long]
}

pub fn build_dimensions() -> Dimensions {
    Dimensions {
        height: height(),
        width: width(),
        length: length(),
    }
}

fn height() -> Size {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, NON_AVERAGE_HEIGHT_CHANGE) {
        let possibilities = non_average_heights();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(height) => *height,
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}

fn length() -> Size {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, NON_AVERAGE_LENGTH_CHANGE) {
        let possibilities = non_average_lengths();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(length) => *length,
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}

fn width() -> Size {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, NON_AVERAGE_WIDTH_CHANGE) {
        let possibilities = non_average_widths();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(width) => *width,
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}
