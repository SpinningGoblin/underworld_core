use rand::{prelude::ThreadRng, Rng};

/**
 * Rolls a d6 multiple times and returns a sum of the result + the modifier.
 */
pub fn roll_d6(rng: &mut ThreadRng, num_rolls: usize, modifier: i32) -> i32 {
    let roll: i32 = (0..num_rolls)
        .map(|_| -> i32 { rng.gen_range(1..=6) })
        .sum();
    0.max(roll + modifier)
}

/**
 * Rolls a d100 multiple times and returns a sum of the result + the modifier.
 */
pub fn roll_d100(rng: &mut ThreadRng, num_rolls: usize, modifier: i32) -> i32 {
    let roll: i32 = (0..num_rolls)
        .map(|_| -> i32 { rng.gen_range(1..=100) })
        .sum();
    0.max(roll + modifier)
}

pub fn roll_percent_succeeds(rng: &mut ThreadRng, percent: i32) -> bool {
    roll_d100(rng, 1, 0) <= percent
}
