use std::collections::HashMap;

fn main() {
    // Rust: Sequence ----> Python:List 
    let seq = [1,2,3,4,5];

    // for n in seq {
    //     print!("{}", n);
    // }

    // Hash Maps
    let mut map = HashMap::new();

    for (i,n) in seq.iter().enumerate() {
        map.insert(n, i);
    }

    // Always need to pattern match a key before accessing it as it may or may not contain the element.
    let random_key= map.get(&5);

    match random_key {
        Some(&n) => println!("Found 5 at index: {}", n),
        None => println!("Value not Found!"),
    };

    for (k,v) in &map {
        println!("{}: {}", k, v);
    }
}