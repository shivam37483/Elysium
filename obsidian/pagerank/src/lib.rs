// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
pub struct Pagerank {
    damping: f64,
    iterations: usize,
}

impl Pagerank {
    pub fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    pub fn rank(&self, graph: Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut rank = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterating over each node and their edges in the graph to identify the contribution made by each node to its respective neighbors
            for (node, edges) in graph.iter().enumerate() {

                // Cal. individual contribution of a node made to its edges(neighbors)
                let contribution = rank[node] / (edges.len() as f64);

                // We then increment the rank of each those neighbors connected to the node by its contribution
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Cal. the net rank by including the randomness of the damping factor
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Updating the ranks of all nodes for next iteration
            rank = new_ranks;
        }

        rank
    }

}

