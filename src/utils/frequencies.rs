use core::hash::Hash;
use std::collections::HashMap;

pub fn sorted_frequencies<T: Iterator<Item = U>, U: Eq + Hash + Clone>(items: T) -> Vec<(U, usize)> {
    let mut counts: HashMap<U, usize> = HashMap::new();

    for item in items {
        *counts.entry(item).or_default() += 1;
    }

    let mut all: Vec<(U, usize)> = counts.iter().map(|(i, j)| (i.clone(), *j)).collect();
    all.sort_by_key(|(_, c)| *c);
    all
}
