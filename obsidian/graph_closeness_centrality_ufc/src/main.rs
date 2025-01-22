use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;


// Allows to use print with - {:?}, Automatically generates code for implementing Debug trait in our struct but since we ourself are implementing the Display trait its ommited.
// #[derive(Debug)]

struct Fighter {
    name: String,
}

impl Fighter {
    // Function takes &str as an input and returns an instance of Fighter
    fn new(name: &str) -> Self {
        // Returns the instance of Fighter with name mapped to String of &str (taken as input) 
        Self {
            name: String::from(name),
        }
    }
}

/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/

// Instead of printing "name: Bruce Lee" we implement Display trait to only display "Bruce Lee" 
impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


fn main() {
    // Initializing an Undirected Graph
    let mut graph = UnGraph::new_undirected();
    
    // Array of all the fighters
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];
        
    // Creating a Vectore storing all the NodeIndex representing all the fighters in Graph as nodes, mapped with the index. 
    // graph.add_node(fighter) adds the fighter to the graph and returns a NodeIndex, which is a unique identifier for the node in the graph. 
    let fighter_nodes = fighters
                        .iter()
                        .map(|fighter| graph.add_node(fighter))
                        .collect();
    // println!("{:?}", fighter_nodes);                    
    // [NodeIndex(0), NodeIndex(1), NodeIndex(2), NodeIndex(3), NodeIndex(4)]
                    
                    
    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz
    
    
    // Printing all the fighters along with their CC where smaller CC means more matches/potency.
    for (i, node) in fighter_nodes.iter().enumerate() {
        // Finding the actual name of the fighter present in the array with the index used in Nodeindex Vector
        let name = &fighters[i];
        
        let degree = graph.edges_directed(*node, Direction::Outgoing).count() as f32;
        
        let closness = 1.0 / degree;
    
        println!("Closeness of {} is {:.2}", name, closness);
    }
}


fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &Vec<NodeIndex>, n1: usize, n2: usize){
    // n1, n2 represent the index of nodes in node index b/w which the edge is created
    graph.add_edge(nodes[n1], nodes[n2], 1.0);
}
