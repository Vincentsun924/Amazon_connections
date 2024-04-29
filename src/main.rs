use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;


//test 

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
    let mut node_connections_formatted = Vec::new();

    // Get the source nodes and their connections
    let node_connections = get_source_connections(&edges, &nodes);

    // Print out the source nodes and their connections
    println!("Source nodes and their connections:");
    for (node, connections) in &node_connections {
        let formatted_connections = connections
            .iter()
            .map(|&dest| (node.clone(), dest)) // Clone node to avoid moving it
            .collect::<Vec<(u32, u32)>>(); // Store as vector of tuples
        node_connections_formatted.push(formatted_connections);
    }
    println!("Nodes: {:?}", nodes);
    println!("Node Connections: {:?}", node_connections_formatted);
}



//test

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

// //finding distances using BFS
// type Vertex = usize;
// type ListOfEdges = Vec<(Vertex,Vertex)>;
// type AdjacencyLists = Vec<Vec<Vertex>>;

// #[derive(Debug)]
// struct Graph {
//     n: usize, // vertex labels in {0,...,n-1}
//     outedges: AdjacencyLists,
// }

// // reverse direction of edges on a list
// fn reverse_edges(list:&ListOfEdges)
//         -> ListOfEdges {
//     let mut new_list = vec![];
//     for (u,v) in list {
//         new_list.push((*v,*u));
//     }
//     new_list
// }

// impl Graph {
//     fn add_directed_edges(&mut self,
//                           edges:&ListOfEdges) {
//         for (u,v) in edges {
//             self.outedges[*u].push(*v);
//         }
//     }
//     fn sort_graph_lists(&mut self) {
//         for l in self.outedges.iter_mut() {
//             l.sort();
//         }
//     }
//     fn create_directed(n:usize,edges:&ListOfEdges)
//                                             -> Graph {
//         let mut g = Graph{n,outedges:vec![vec![];n]};
//         g.add_directed_edges(edges);
//         g.sort_graph_lists();
//         g                                        
//     }
    
//     fn create_undirected(n:usize,edges:&ListOfEdges)
//                                             -> Graph {
//         let mut g = Self::create_directed(n,edges);
//         g.add_directed_edges(&reverse_edges(edges));
//         g.sort_graph_lists();
//         g                                        
//     }
// }
// fn adjacency_list(){
//     let n: usize = 10;
//     let mut edges: ListOfEdges = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
//     edges.sort();
//     println!("{:?}",edges);
//     let graph = Graph::create_undirected(n,&edges);
//     for (i, l) in graph.outedges.iter().enumerate() {
//         println!("{} {:?}", i, *l);
//     }
// }

// fn compute_distance(){
//     let start: Vertex = 2; // <= we'll start from this vertex

//     let mut distance: Vec<Option<u32>> = vec![None;graph.n];
//     distance[start] = Some(0); // <= we know this distance

//     use std::collections::VecDeque;
//     let mut queue: VecDeque<Vertex> = VecDeque::new();
//     queue.push_back(start);

//     print!("vertex:distance");
//     for v in 0..graph.n {
//         print!("   {}:{}",v,distance[v].unwrap());
//     }
//     println!();
// }

