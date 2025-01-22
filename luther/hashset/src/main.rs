use rand::thread_rng;
use rand::seq::SliceRandom;

use std::collections::HashSet;


fn generate_fruit() -> String {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];

    let mut rng = thread_rng();

    match fruits.choose(&mut rng) {
        Some(s) => return s.to_string(),
        _ => return String::from(""),
    }
}

fn main() {
    let mut hashset = HashSet::new();
    for _ in 0..10 {
        hashset.insert(generate_fruit());
    }

    println!("Len is: {}, actual no is: {:?}",hashset.len(), hashset);
}
