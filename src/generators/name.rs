use rand::Rng;

use crate::utils::rolls::roll_percent_succeeds;

const CHANCE_UNNAMED: i32 = 10;

pub fn generate_name() -> Option<String> {
    let mut rng = rand::thread_rng();
    if roll_percent_succeeds(&mut rng, CHANCE_UNNAMED) {
        return None;
    }

    let num_parts: usize = rng.gen_range(1..=3);

    let mut all_name_parts = name_parts();
    let name_parts: Vec<&str> = (0..num_parts)
        .map(|_| {
            let index = rng.gen_range(0..all_name_parts.len());
            all_name_parts.remove(index)
        })
        .collect();

    Some(name_parts.join(""))
}

fn name_parts() -> Vec<&'static str> {
    vec![
        "gon", "gro", "grub", "num", "gorg", "zerg", "and", "por", "mer", "mog", "og", "zola",
        "mar", "ar", "la", "zo", "mank", "mang", "grap", "log", "lorg", "glor", "bor", "bob",
        "plo", "nor", "norn", "mac", "oor", "onk", "rok", "ron", "car", "toe", "pan", "flick",
        "lerg", "wod", "saw", "grum", "crum", "rin", "bal", "rog",
    ]
}
