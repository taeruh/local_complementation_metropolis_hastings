use std::collections::HashSet;

#[derive(Clone)]
pub struct Graph {
    nodes: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let mut nodes = Vec::with_capacity(size);
        for _ in 0..size {
            nodes.push(HashSet::new());
        }
        Graph { nodes }
    }

    pub fn get_data(&self) -> &Vec<HashSet<usize>> {
        &self.nodes
    }

    pub fn add_edge_unchecked(&mut self, a: usize, b: usize) {
        self.nodes[a].insert(b);
        self.nodes[b].insert(a);
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        if a >= self.nodes.len() || b >= self.nodes.len() {
            panic!("Node index out of bounds: {a} or {b}");
        } else if a == b {
            panic!("Cannot add edge from node {a} to itself");
        } else {
            self.add_edge_unchecked(a, b);
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_neighbours(&self, node: usize) -> Option<&HashSet<usize>> {
        self.nodes.get(node)
    }
}
