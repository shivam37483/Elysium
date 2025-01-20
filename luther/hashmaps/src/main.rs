use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();

    m.insert(3,4);

    match m.get(&5) {
        Some(v) => println!("yrreeee"),
        None => println!("trash"),
    };
}

