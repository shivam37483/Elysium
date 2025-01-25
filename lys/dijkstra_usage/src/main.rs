use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {

    // Aim is to find shortest dist b/w 2 nodes

    let mut graph = Graph::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    
    // Creates a map/vector-index to store all the shortest dist. corresponding the end point, all measured from a single starting point. Start was not Some() coz it expects node ID directly not an option, we wrote erronous it wont compile.
    let node_map = dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight());
    
    // Matching whther the end point mentioned is actually present in Node-Index
    if let Some(distance) = node_map.get(&lisbon_cathedral) {
        println!("Shortest distance is {}", distance)
    } else {
        print!("No Shortest Distance found")
    }
    
}
