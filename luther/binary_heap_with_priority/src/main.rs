use rand::thread_rng;
use rand::seq::SliceRandom;

use std::collections::BinaryHeap;

use std::cmp::{Ord, Ordering};

// Binary Heap in Rust ---> Max(Default) Heap in Python.Aim is to create a salad which has fig with highest priority.

// High priority ensures these elements have faster access times whilts maintaining the order. 

// Create a enum which will help us classify fruits as Options. since our aim is to assign fig as higher priority thru enum we can easily filter out all the fruits that are not fig.

// Both Ord and PartialOrd are required by Binary Heap for implemention in order to determine priority of all the elements(Strings?)
#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

// Implementing Ord trait allows us to create an ordering which states that one value is greater/less than/equal to the other value in the type.
impl Ord for Fruit {
    // Requires the implementaion of cmp which returns an ordering. It takes 2 references of Fruit, and returns their respective ordering value (Less, Equal, or Greater).
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Handling all the possible cases for different refernces of Fruit type
        match (self, other) {
            (Fruit::Fig, Fruit::Other(_))      => Ordering::Greater,
            (Fruit::Fig, Fruit::Fig)           => Ordering::Equal,
            (Fruit::Other(_), Fruit::Fig)      => Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => Ordering::Equal,
        }
    }
}

// Partial Orde is mandatory to be implemented by Eq. Partial Order is more flexible?
impl PartialOrd for Fruit {
    // Required by Partial order, returns an Option.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Since we already implemented ord above we recycle its code
        Some(self.cmp(other))
    }
}


fn generate_fruits() -> BinaryHeap<Fruit> {
    let mut fruit_salad = BinaryHeap::new();

    let fruits = vec![
        String::from("Apple"),
        String::from("Orange"),
        String::from("Pear"),
        String::from("Peach"),
        String::from("Banana"),
        String::from("Fig"),
        String::from("Fig"),
        String::from("Fig"),
        String::from("Fig"),
    ];

    let mut rng = thread_rng();

    let mut fig_count = 0;

    // We wish to add random fruits until we hv atleast 2 figs in salad
    while fig_count < 2 {
        let fruit = fruits.choose(&mut rng);
        
        // Matching for validity if fruits
        match fruit {
            // If the fruit is fig then append Fruit::fig to heap
            Some(f) if *f == "Fig" =>  {
                fig_count += 1 ;
                fruit_salad.push(Fruit::Fig);
            }

            // Else random ahh fruit
            Some(f) => fruit_salad.push(Fruit::Other(f.clone())),

            // None case
            _ => println!("No fruit")
        };
    }

    fruit_salad
}

// Better implementation
fn generate_fruits2() -> BinaryHeap<Fruit> {
    let mut fruit_salad = BinaryHeap::new();

    let fruits = vec![
        String::from("Apple"),
        String::from("Orange"),
        String::from("Pear"),
        String::from("Peach"),
        String::from("Banana"),
        String::from("Fig"),
        String::from("Fig"),
        String::from("Fig"),
        String::from("Fig"),
    ];

    let mut rng = thread_rng();

    let mut fig_count = 0;

    while fig_count < 2 {
        // Instead of matching we use simple if else conditionals to determine whther we hv a value of fruits
        if let Some(f) = fruits.choose(&mut rng) {
            if f == "Fig" {
                fig_count += 1;
                fruit_salad.push(Fruit::Fig);
            } else {
                fruit_salad.push(Fruit::Other(f.clone()));
            }
        } else {
            print!("No fruit")
        }
    }

    fruit_salad
}


fn main() {
    let salad = generate_fruits();
    println!("{:?}", salad);

    let salad2 = generate_fruits2();
    println!("{:?}", salad2);
}
