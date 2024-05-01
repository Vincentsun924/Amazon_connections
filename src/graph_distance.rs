use std::collections::VecDeque;

// Define types
pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

// Define the Graph struct
#[derive(Debug)]
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}

impl Graph {
    // Method to add directed edges
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    // Method to sort graph lists
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    // Method to create directed graph
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph { n, outedges: vec![vec![]; n] };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

// Function to compute distances using BFS
pub fn compute_and_print_distance_bfs(start: Vertex, graph: &Graph) -> Vec<(usize, u32)> {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0); // Set the distance of the start node to 0
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);

    // Perform BFS to compute distances
    while let Some(v) = queue.pop_front() {
        for &u in &graph.outedges[v] {
            if distance[u].is_none() {
                distance[u] = Some(distance[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }
    // Return distances that are connected to the 5 nodes picked randomly
    let mut result = Vec::new();
    for (i, &dist) in distance.iter().enumerate() {
        if let Some(d) = dist {
            result.push((i, d));
        }
    }
    result
}
