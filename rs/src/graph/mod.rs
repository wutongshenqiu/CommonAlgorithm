use std::fmt;
use std::fs;

mod algorithms;
mod utils;


pub struct Edge {
    in_vertex: usize,
    out_vertex: usize,
    weight: i32,
}

impl Edge {
    pub fn new(in_vertex: usize, out_vertex: usize, weight: i32) -> Edge {
        Edge {
            in_vertex,
            out_vertex,
            weight
        }
    }

    pub fn to_str(&self) -> String {
        format!("in vertex: {}, out vertex: {}, weight: {}", self.in_vertex, self.out_vertex, self.weight)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "in vertex: {}, out vertex: {}, weight: {}", self.in_vertex, self.out_vertex, self.weight)
    }
}


pub struct EdgeIterator<'a> {
    graph: &'a AdjacencyList,
    next_edge_index: i32,
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a Edge;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_edge_index {
            -1 => None,
            x if x > 0 => {
                let current_edge = self.graph.edges[self.next_edge_index as usize].as_ref();
                let new_next_edge_index = self.graph.next[self.next_edge_index as usize];
                self.next_edge_index = new_next_edge_index;
                current_edge
            },
            _ => panic!("Index of edge is not allowed!")
        }
    }
}

/// more info can be found in [邻接表](https://wiki.jikexueyuan.com/project/easy-learn-algorithm/clever-adjacency-list.html)
pub struct AdjacencyList {
    /// suppose the index of vertex and edge are started from 1 but not 0
    edges: Vec<Option<Edge>>,
    first: Vec<i32>,
    next: Vec<i32>,

    num_vertices: usize,
    num_edges: usize,

    /// current number of edges in AdjacencyList
    _current_num_edges: usize,
}

impl AdjacencyList {
    pub fn new(num_vertices: usize, num_edges: usize) -> AdjacencyList {
        AdjacencyList {
            edges: {
                let mut edges = Vec::with_capacity(num_edges+1);
                edges.resize_with(num_edges+1, || None::<Edge>);
                edges
            },
            first: vec![-1; num_vertices+1 as usize],
            next: vec![-1; num_edges+1 as usize],

            num_vertices,
            num_edges,

            _current_num_edges: 0,
        }
    }

    pub fn num_vertices(&self) -> usize{
        self.num_vertices
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    pub fn add_directed_edge(&mut self, in_vertex: usize, out_vertex: usize, weight: i32) {
        if in_vertex <= 0 || out_vertex <= 0 {
            panic!("numerical order of vertex can not be less than 0!");
        }
        if in_vertex > self.num_vertices || out_vertex > self.num_vertices {
            panic!("numerical order of vertex can not be greater than {}!", self.num_vertices);
        } 
        if self._current_num_edges >= self.num_edges {
            panic!("can not add edge to AdjacencyList because it has already full!");
        }
        self._current_num_edges += 1;

        self.edges[self._current_num_edges] = Some(Edge::new(in_vertex, out_vertex, weight));
        self.next[self._current_num_edges] = self.first[in_vertex];
        self.first[in_vertex] = self._current_num_edges as i32;
    }

    pub fn add_undirected_edge(&mut self, in_vertex: usize, out_vertex: usize, weight: i32) {
        self.add_directed_edge(in_vertex, out_vertex, weight);
        self.add_directed_edge(out_vertex, in_vertex, weight);
    }

    pub fn edge_iterator_of_vertex(&self, vertex: usize) -> EdgeIterator {
        EdgeIterator {
            graph: self,
            next_edge_index: self.first[vertex],
        }
    }
}


impl fmt::Display for AdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut all_edge_strs = String::new();
        for edge_index in 1..(self.edges.len()) {
            all_edge_strs.push_str(self.edges[edge_index].as_ref().unwrap().to_str().as_str());
            all_edge_strs.push('\n');
        }

        write!(f, "{}", all_edge_strs)
    }
}


pub fn load_adj_list_from_file(file_name: &str) -> AdjacencyList{
    let content = fs::read_to_string(file_name)
        .expect(&format!("fail to read file: `{}`", file_name));

    let mut lines = content.lines();
    
    let first_line = lines.next().unwrap();
    let first_line: Vec<&str> = first_line.split_ascii_whitespace().collect();
    let num_vertices = first_line[0].parse::<usize>().unwrap();
    let num_edges = first_line[1].parse::<usize>().unwrap();
    
    let mut graph = AdjacencyList::new(
        num_vertices,
        num_edges
    );

    for line in lines {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let in_vertex = line[0].parse::<usize>().unwrap();
        let out_vertex = line[1].parse::<usize>().unwrap();
        let weight = line[2].parse::<i32>().unwrap();

        graph.add_directed_edge(in_vertex, out_vertex, weight);
    }

    graph
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRAPH_FILE: &str = "src/graph/examples/test.txt";

    #[test]
    fn test_load_graph_from_file() {
        let adj_list = load_adj_list_from_file(TEST_GRAPH_FILE);
        assert_eq!(adj_list.num_edges(), 5);
        assert_eq!(adj_list.num_vertices(), 4);
    }

    #[test]
    fn test_edge_iterator() {
        let adj_list = load_adj_list_from_file(TEST_GRAPH_FILE);
        let mut edge_iterator = adj_list.edge_iterator_of_vertex(1);
        let edge = edge_iterator.next().unwrap();
        assert_eq!((edge.in_vertex, edge.out_vertex, edge.weight), (1, 4, 8));
        let edge = edge_iterator.next().unwrap();
        assert_eq!((edge.in_vertex, edge.out_vertex, edge.weight), (1, 2, 4));
        let edge = edge_iterator.next();
        assert!(edge.is_none());
    }

}