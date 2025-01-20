// Rand is a random num. generator lib in Rust
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    // Rust: Sequence/Vectors ----> Python:List 
    
    // This creates an array of fixed size 5, which is deployed on Stack
    // let seq = ["Apple", "Orange", "Banana", "Pear", "Keenu"];

    // Creates a growable array, allocated on heap
    let mut fruit = vec!["Apple", "Orange", "Banana", "Pear", "Keenu"];

    // Creates an instance of a rand which can be used in shuffling the sequences
    let mut rng = thread_rng();

    // Shuffles the elements present in the array itself (For which the arr needs to be mutable)
    fruit.shuffle(&mut rng);

    for f in fruit {
        print!("{} ", f)
    }
    
}