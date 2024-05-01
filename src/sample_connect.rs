use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use std::collections::VecDeque;


pub fn samples(edges: &Vec<(u32, u32)>) -> Vec<u32> {
    // Shuffle the edges randomly
    let mut rng = rand::thread_rng();
    let mut shuffled_edges = edges.clone();
    shuffled_edges.shuffle(&mut rng);

    // Extract unique source nodes from the random sample
    let mut nodes = Vec::new();
    for &(src, _) in shuffled_edges.iter().take(5) {
        if !nodes.contains(&src) {
            nodes.push(src);
        }
    }
    nodes.sort();
    nodes
}
//get the nodes where each 5 selected random nodes are connected to and push it into a vector called node_connections
pub fn get_source_connections(edges: &Vec<(u32, u32)>, nodes: &Vec<u32>) -> Vec<(u32, Vec<u32>)> {
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