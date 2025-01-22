use std::collections::HashMap;

// Aim is to create a Frequence counter which gives the count of each element in the array/vector.

// Rust provides O(1) complexity or insertion, access, deletion in Hashmaps

fn countr(nums: Vec<i32>) -> Vec<(i32, i32)> {
    let mut hashmap = HashMap::new();

    for n in nums {
        // First we get the current count of the n in Hashmap and if the n was not previously present place 0 for its count. 
        let count = hashmap.entry(n).or_insert(0);

        // Then we the append the value of that count in the Hashmap by 1
        *count += 1;
    }

    let mut res = Vec::new();

    for (k,v) in hashmap {
        res.push((k,v));
    }

    res
}

fn main() {
    let vec = vec![2,24,3,5,3,5,6,3,2,4,5,6,5,3,34,5,6,65,65,43,3,43,45,45,45,45,5,5,53,3,32,32,54,656,7];

    println!("{:?}", countr(vec));
}