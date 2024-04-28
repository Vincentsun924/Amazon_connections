use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;

//first

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
    let node_connections = get_connection(&edges, &nodes);

    // Print out the random sample of nodes and their connections
    println!("Random sample of nodes and their connections:");
    for (node, connections) in &node_connections {
        println!("Node {}: {:?}", node, connections);
    }
}

fn samples(edges: &Vec<(u32, u32)>) -> Vec<u32> {
    // Shuffle the edges randomly
    let mut rng = rand::thread_rng();
    let mut shuffled_edges = edges.clone();
    shuffled_edges.shuffle(&mut rng);

    // Take the first 10 edges as a random sample
    let random_sample = &shuffled_edges[..std::cmp::min(10, shuffled_edges.len())];

    // Extract unique nodes from the random sample
    let mut nodes = Vec::new();
    for &(src, dest) in random_sample {
        nodes.push(src);
        nodes.push(dest);
    }
    nodes.sort();
    nodes.dedup();
    nodes
}

fn get_connection(edges: &Vec<(u32, u32)>, nodes: &Vec<u32>) -> Vec<(u32, Vec<u32>)> {
    let mut node_connections = Vec::new();
    for &node in nodes {
        let connections: Vec<u32> = edges
            .iter()
            .filter_map(|&(src, dest)| if src == node { Some(dest) } else { None })
            .chain(edges.iter().filter_map(|&(src, dest)| if dest == node { Some(src) } else { None }))
            .collect();
        node_connections.push((node, connections));
    }
    node_connections
}
