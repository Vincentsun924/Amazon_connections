use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use std::collections::VecDeque;

fn read_file(path: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        if let Ok(line_str) = line {
            let v: Vec<&str> = line_str.trim().split('\t').collect();
            if v.len() >= 2 {
                if let (Ok(x), Ok(y)) = (v[0].parse::<u32>(), v[1].parse::<u32>()) {
                    result.push((x, y));
                }
            }
        }
    }
    result
}

fn main() {
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

    // Compute distances
    let nodes: Vec<Vertex> = nodes.iter().map(|&x| x as Vertex).collect();
    compute_distance(&graph, nodes);
}

// Test

fn samples(edges: &Vec<(u32, u32)>) -> Vec<u32> {
    // Shuffle the edges randomly
    let mut rng = rand::thread_rng();
    let mut shuffled_edges = edges.clone();
    shuffled_edges.shuffle(&mut rng);

    // Extract unique source nodes from the random sample
    let mut nodes = Vec::new();
    for &(src, _) in shuffled_edges.iter().take(2) {
        if !nodes.contains(&src) {
            nodes.push(src);
        }
    }
    nodes.sort();
    nodes
}

fn get_source_connections(edges: &Vec<(u32, u32)>, nodes: &Vec<u32>) -> Vec<(u32, Vec<u32>)> {
    let mut node_connections = Vec::new();
    for &node in nodes {
        let connections: Vec<u32> = edges
            .iter()
            .filter_map(|&(src, dest)| if src == node { Some(dest) } else { None })
            .collect();
        node_connections.push((node, connections));
    }
    node_connections
}

// Finding distances using BFS
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

impl Graph {
    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph { n, outedges: vec![vec![]; n] };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

fn compute_distance(graph: &Graph, start_nodes: Vec<usize>) {
    // Keep track of visited nodes
    let mut visited = vec![false; graph.n];

    // Initialize the queue with start nodes
    let mut queue = VecDeque::new();
    for &start_node in &start_nodes {
        visited[start_node] = true;
        queue.push_back((start_node, 0)); // Start nodes are at distance 0
    }

    println!("Vertex: Distance");
    // BFS traversal
    while let Some((node, distance)) = queue.pop_front() {
        println!("{}: {}", node, distance);
        // Explore neighbors of the current node
        for &neighbor in &graph.outedges[node] {
            // If the neighbor has not been visited yet
            if !visited[neighbor] {
                visited[neighbor] = true;
                // Add the neighbor to the queue with updated distance
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
}
