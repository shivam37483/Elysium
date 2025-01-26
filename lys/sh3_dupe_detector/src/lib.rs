use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

use sha3::{Digest, Sha3_256};

use std::collections::HashMap;


// Static in order to initialize as Global var.
static PHRASES: [&str; 10] = [
    "man can be destroyed but not defeated",
    "but man is not made for defeat",
    "a man can be destroyed but not defeated",
    "the old man was thin and gaunt",
    "everything about him was old",
    "the sail was patched with flour sacks",
    "he was an old man who fished alone",
    "the old man had taught the boy to fish",
    "the old man looked at him with his sun burned confident loving eyes",
    "his eyes were cheerful and undefeated",
];


// Take each phrase create its copies for a random no of times and then append into returning array
pub fn generate_random_phrases() -> Vec<String> {
    let mut rng = thread_rng();
    let mut phrases = Vec::new();

    for phrase in PHRASES {
        let copies = rng.gen_range(1..4000);

        for _ in 0..copies{
            phrases.push(phrase.to_string());
        }
    }

    phrases.shuffle(&mut rng);

    phrases
}


// Create an hashmap and store each unique phrase(along with its count) corresponding to its 256 hash 
pub fn analyze_duplicates(phrases: Vec<String>) {
    let mut hashes = HashMap::new();

    for phrase in phrases {
        // Create the hash for the corresponding phrase
        let hash = Sha3_256::digest(&phrase);

        // If entry is already present fetch it else create a new entry corresponding to the hash with the resp. phrase and count as 0 
        let entry = hashes.entry(hash).or_insert((0, phrase));

        // Increment the count for each phrase
        entry.0 += 1;
    }


    let total_unique_phrases = hashes.len();

    // This is the count of phrases that appear more than once in the list. Each phrase that has duplicates is counted once.
    let mut total_unique_duplicates = 0;

    // This is the total number of duplicate instances across all phrases. For each phrase that appears more than once, this count includes all occurrences except the first one(orignal).
    let mut total_combined_duplicates = 0;


    for (hash,(count, phrase)) in hashes {
        if count > 1 {
            total_unique_duplicates += 1;
            total_combined_duplicates += count-1;

            println!("{} - {} times: {}", hex::encode(hash), count, phrase);
        }
    }

    println!("Total Unique Phrases: {}", total_unique_phrases);
    println!("Total Unique Duplicates: {}", total_unique_duplicates);
    println!("Total Combined Duplicates: {}", total_combined_duplicates);
}




