use sh3_dupe_detector::{analyze_duplicates, generate_random_phrases};

fn main() {
    // Aim is to from a set of given phrases, duplicate the phrases in a new vector then count each of the phrases' occureance along with their sha3 generated hash

    let phrases = generate_random_phrases();

    analyze_duplicates(phrases);

}
