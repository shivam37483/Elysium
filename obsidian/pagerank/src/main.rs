use pagerank::Pagerank;

fn main() {
    
    // Here each index represents a node and vec present at each index is its neighbors to which its connected 
    let graph = vec![
        vec![1, 2],  // ESPN links to NFL, NBA
        vec![0],     // NFL links to ESPN
        vec![0, 3],  // NBA links to ESPN, UFC
        vec![0],     // UFC links to ESPN
        vec![0, 1],  // MLB links to ESPN, NFL
    ];


    // The names corresponding to the indexes of the websites.
    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // Creating an instance of the Pagerank struct
    let pagerank = Pagerank::new(0.85, 1111);

    
    let ranks = pagerank.rank(graph);

    // Dislpaying the Output
    for i in 0..ranks.len() {
        println!("Rank of {} is: {}",names[i], ranks[i])
    }
}
