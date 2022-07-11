use rand::Rng;

use crate::{
    components::{rooms::Dimensions, size::Size},
    utils::rolls::roll_d100,
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
    let non_average_roll = roll_d100(&mut rng, 1, 0);
    if non_average_roll <= NON_AVERAGE_HEIGHT_CHANGE {
        let possibilities = non_average_heights();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(height) => height.clone(),
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}

fn length() -> Size {
    let mut rng = rand::thread_rng();
    let non_average_roll = roll_d100(&mut rng, 1, 0);
    if non_average_roll <= NON_AVERAGE_LENGTH_CHANGE {
        let possibilities = non_average_lengths();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(length) => length.clone(),
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}

fn width() -> Size {
    let mut rng = rand::thread_rng();
    let non_average_roll = roll_d100(&mut rng, 1, 0);
    if non_average_roll <= NON_AVERAGE_WIDTH_CHANGE {
        let possibilities = non_average_widths();
        let index = rng.gen_range(0..possibilities.len());
        match possibilities.get(index) {
            Some(width) => width.clone(),
            None => Size::Average,
        }
    } else {
        Size::Average
    }
}
