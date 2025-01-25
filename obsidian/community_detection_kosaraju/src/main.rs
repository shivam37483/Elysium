use community_detection_kosaraju::TWITTER_USERNAMES;

use petgraph::algo::kosaraju_scc;

use petgraph::prelude::*;
use std::collections::HashMap;


fn main() {

    // Our aim is to create a directed graph with all the respective nodes and edges and then dump it into kosaraju algo which will then return to us all the strongly connected nodes.

    let mut graph = DiGraph::new();

    // Create a Hashmap to store all the nodeindex corresponding to their usernames. The main use of hashmap here is to avoid duplicate nodes for same username i.e each username should hv a single node corresponding to it. If we dont use a hashmap grpgh will create a new node for each recurring username.
    // username ----> NodeIndex
    let mut nodes = HashMap::new();

    
    // Iterate over each username to populate the hashmap and the graph. We create a window which states for an inpur arr = [1,2,3] it will return (win =2): [[1,2], [2,3], [3,3]] 
    for window in TWITTER_USERNAMES.windows(2) {

        // Dataset states that each of these user is retweeting with the mention of another user in their threads. 
        let user = window[0]; 
        let mention = window[1];

        // For each user and its mention we populate the Hashmap. If the entry is already created we access it else we create a node in the graph and populate the hashmap by creating a new entry of that username as key and value as default value(reference to nodeindex) returned by .add_node fn of graph. 
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));

        let mention_node = *nodes.entry(mention).or_insert_with(|| graph.add_node(mention));

        // For each newly added user and its mention we create a directed edge in the graph with the wt as - retweets
        graph.add_edge(user_node, mention_node, "retweets");
    }

    // We intitialize the kosa algo with our graph, which then identifies all the strongly connected nodes and groups them as a chunk called - components
    let kosa = kosaraju_scc(&graph);

    // We then iterate over each component and print out its grouped nodes
    for component in kosa {
        println!("These are the Strongly connected communities: {}", component.len());

        // For each corresponding node in a component we map nodeindex in the graph to fetch its corresponding username. In graph each node stores - nodeindex, data(in our case username: &str), when we call map(|&node_index| graph[node_index]) it first derefrences the nodeindex provided by the component and then map it to its corresponding username stored in the node
        let usernames: Vec<_> = component.iter().map(|&node_index| graph[node_index]).collect();

        print!("{:?} ", usernames);
        println!();
    }
}
