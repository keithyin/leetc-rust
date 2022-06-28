use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

struct Graph {
    graph: HashMap<String, Vec<Edge>>
}

impl Graph {
    pub fn new() -> Self {
        Graph{graph: HashMap::new()}
    }

    pub fn add(&mut self, from: String, to: String) {
        if let Some(edges) = self.graph.get_mut(&from) {
            edges.push(Edge::new(from, to));
        } else {
            self.graph.insert(from.clone(), vec![Edge::new(from, to)]);
        }
    }

}

#[derive(Clone, Copy)]
struct Edge {
    from: String,
    to: String,
    weight: i32
}

impl Edge {
    pub fn new(from: String, to: String) -> Self {
        Edge{from, to, weight: 1}
    }
}

struct ShortestPath {
    graph: Graph,
    source: String,
    dist_to: HashMap<String, i32>,
    path_to: HashMap<String, Edge>,
}


impl ShortestPath {
    pub fn new(graph: Graph, source: String) -> Self {
        ShortestPath{graph, source, dist_to: HashMap::new(), path_to: HashMap::new()}
    }


}


pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {

    "".cmp()
}