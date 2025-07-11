//import the read_file function from the read.rs module
mod read;
use read::read_file;

mod sample_connect;
use sample_connect::{samples, get_source_connections};

// import the graph_distance module into scope
mod graph_distance;
// Use the types and functions from the graph_module
use graph_distance::{Graph, Vertex, ListOfEdges, compute_and_print_distance_bfs};

fn main() {
    //use the read_file function in read.rs
    let edges = read_file("amazon.txt");
    let nodes = samples(&edges);

    // Calculate the maximum node number
    let max_node = edges.iter().map(|&(src, dest)| src.max(dest)).max().unwrap_or(0);
    let n = max_node as usize + 1;

    let mut node_connections_formatted = Vec::new();

    // Get the source nodes and their connections
    let node_connections = get_source_connections(&edges, &nodes);

    // Print out the source nodes and their connections
    println!("Source nodes and their connections:");
    for (node, connections) in &node_connections {
        let formatted_connections = connections
            .iter()
            .map(|&dest| (*node, dest)) // Source node is not cloned
            .collect::<Vec<(u32, u32)>>(); // Store as vector of tuples
        node_connections_formatted.extend(formatted_connections);
    }
    println!("Nodes: {:?}", nodes);
    println!("Node Connections: {:?}", node_connections_formatted);

    // Print out the adjacency list
    println!("Adjacency List:");
    let mut edges_flat = node_connections_formatted
        .iter()
        .map(|&(src, dest)| (src as usize, dest as usize))
        .collect::<ListOfEdges>();
    edges_flat.sort();
    let graph = Graph::create_directed(n, &edges_flat); // Create a directed graph
    for (i, l) in graph.outedges.iter().enumerate() {
        if !l.is_empty() { // Print only if the connection list is not empty
            println!("{}: {:?}", i, l);
        }
    }
// Compute distances from the original two nodes
   let mut total_distance = 0;
   let mut total_nodes = 0;
   for &start_node in &nodes {
       let distances = compute_and_print_distance_bfs(start_node as Vertex, &graph);
       for (node, dist) in distances {
           println!("{}: {}", node, dist);
           total_nodes += 1;
           total_distance += dist;
       }
   }
   // Compute and print the average distance
   let average_distance = total_distance as f32 / total_nodes as f32;
   println!("Average distance from original nodes: {}", average_distance);
}


//test that the 5 selected nodes each have at least something connected to it
use std::env;


#[test]
fn test_nodes_connections() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the path to "amazon.txt" relative to the current directory
    let file_path = current_dir.join("src").join("amazon.txt");

    // Convert the path to a string
    let file_path_str = file_path.to_str().expect("Failed to convert path to string");

    // Use the read_file function with the correct file path
    let edges = read_file(file_path_str);
    let nodes = samples(&edges);

    // Get the source nodes and their connections
    let node_connections = get_source_connections(&edges, &nodes);

    // Assert that each of the five nodes has at least one connection
    for (node, connections) in &node_connections {
        assert!(nodes.contains(node), "Node {} not found in the sampled nodes", node);
        assert!(!connections.is_empty(), "Node {} should have at least one connection", node);
    }
}




#[test]
fn test_compute_and_print_distance_bfs() {
    // Get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Construct the path to "amazon.txt" relative to the current directory
    let file_path = current_dir.join("src").join("amazon.txt");

    // Convert the path to a string
    let file_path_str = file_path.to_str().expect("Failed to convert path to string");

    // Use the read_file function with the correct file path
    let edges = read_file(file_path_str);
    let nodes = samples(&edges);

    // Calculate the maximum node number
    let max_node = edges.iter().map(|&(src, dest)| src.max(dest)).max().unwrap_or(0);
    let n = max_node as usize + 1;

    // Get the source nodes and their connections
    let node_connections = get_source_connections(&edges, &nodes);

    // Create a directed graph
    let mut edges_flat = ListOfEdges::new();
    for &(src, ref connections) in &node_connections {
        for &dest in connections {
            edges_flat.push((src as usize, dest as usize));
        }
    }
    edges_flat.sort();
    let graph = Graph::create_directed(n, &edges_flat);

    // For each node, compute distances using BFS
    for &node in &nodes {
        let distances = compute_and_print_distance_bfs(node as Vertex, &graph);

        // Verify that the distance from the node to itself is zero
        let self_distance = distances.iter().find(|&&(n, _)| n == node as usize);
        assert_eq!(self_distance.unwrap().1, 0);

        // Verify that all other distances are greater than zero
        for &(n, dist) in &distances {
            if n != node as usize {
                assert!(dist > 0);
            }
        }
    }
}