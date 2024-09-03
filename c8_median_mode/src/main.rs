use core::hash::Hash;
use std::collections::hash_map::HashMap;

fn mode<T: Eq + Hash + Clone>(values: &[T]) -> Option<T> {
    if values.is_empty() {
        return None;
    }

    let mut occurences: HashMap<T, u32> = HashMap::new();

    for value in values {
        *occurences.entry(value.clone()).or_insert(0) += 1;
    }

    occurences
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(a, _b)| a)
}

fn median<T: Ord + Clone>(values: &[T]) -> Option<T> {
    if values.is_empty() {
        return None;
    };

    let mut values_copy = values.to_vec();
    values_copy.sort();

    Some(values_copy[values_copy.len() / 2].clone())
}

fn main() {
    let values: Vec<u32> = [13, 0, 28, 11, 45, 27, 46, 38, 30, 41, 14, 25, 6, 27, 24].to_vec();

    println!("{}, {}", median(&values).unwrap(), mode(&values).unwrap());
}
