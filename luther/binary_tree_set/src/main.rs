use std::collections::BTreeSet;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[allow(clippy::clone_on_copy)]
fn main() {
    // BTree-set is used to store the val in sorted order, with providing log access, insertion, deletion time.

    let fruits = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];

    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    // Aim is to show how A Single BTreeset stores the values in sorted order based on varying length and input.
    for amt in amounts {
        let mut btreeset = BTreeSet::new();
        
        // Shuffling the fruits
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for f in shuffled_fruits{
            btreeset.insert(f);

            // Maintaining the length
            if btreeset.len() == amt {
                break;
            }
        }

        println!("amount: {}, Val: {:?}", amt, btreeset);
    }
}
