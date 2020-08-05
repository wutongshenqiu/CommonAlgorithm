use super::AdjacencyList;
use super::load_adj_list_from_file;

use std::collections::VecDeque;
use std::collections::BinaryHeap;

use std::cmp::Ordering;

use std::fmt;


pub struct Graph {
    adj_list: AdjacencyList,
}

/// ways to create graph
impl Graph {
    pub fn new(adj_list: AdjacencyList) -> Graph {
        Graph{
            adj_list
        }
    }

    pub fn create_from_file(file_path: &str) -> Graph {
        let adj_list = load_adj_list_from_file(file_path);
        Graph {
            adj_list
        }
    }
}

/// graph alogorithms
impl Graph {
    pub fn dfs(&self) {
        let mut visited = vec![false; self.adj_list.num_vertices()+1];
        let mut stack = Vec::with_capacity(self.adj_list.num_vertices());

        for i in 1..visited.len() {
            if !visited[i] {
                visited[i] = true;
                stack.push(i);
            }
            while !stack.is_empty() {
                let vertex = stack.pop().unwrap();
                // do something before visited all adjacent points
                // println!("visited vertex: {}", vertex);
                let edge_iterator = self.adj_list.edge_iterator_of_vertex(vertex);
                for edge in edge_iterator {
                    let current_vertex = edge.out_vertex;
                    if !visited[current_vertex] {
                        visited[current_vertex] = true;
                        stack.push(current_vertex);
                    }
                }
            }
            // do something when after visited all adjacent points
        }
    }

    pub fn bfs(&self) {
        let mut visited = vec![false; self.adj_list.num_vertices()+1];
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(self.adj_list.num_vertices());
        
        for i in 1..visited.len() {
            if !visited[i] {
                visited[i] = true;
                queue.push_back(i);
            }
            while !queue.is_empty() {
                let vertex = queue.pop_front().unwrap();
                // do something before visited all adjacent point
                // println!("vertex: {}", vertex);
                let edge_iterator = self.adj_list.edge_iterator_of_vertex(vertex);
                for edge in edge_iterator {
                    let current_vertex = edge.out_vertex;
                    if !visited[current_vertex] {
                        visited[current_vertex] = true;
                        queue.push_back(current_vertex);
                    }
                }
            }
            // do something when after visited all adjacent points
        }
    }

    pub fn dijkstra(&self, vertex: usize) {
        // used for heap
        struct Edge {
            vertex: usize,
            weight: i32,
        }

        impl PartialEq for Edge {
            fn eq(&self, other: &Self) -> bool {
                self.weight == other.weight
            }
        }

        impl Eq for Edge {}

        impl PartialOrd for Edge {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Edge {
            fn cmp(&self, other: &Self) -> Ordering {
                // flipping the ordering on weight because heap in standard library is max-heap
                other.weight.cmp(&self.weight)
            }
        }

        impl fmt::Display for Edge {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "vertex: {}, weight: {}", self.vertex, self.weight)
            }
        }
        
        // todo
        // https://stackoverflow.com/questions/14252582/how-can-i-use-binary-heap-in-the-dijkstra-algorithm
        // maybe we need heap with map
        // or just skip but need more allocate
        let mut heap: BinaryHeap<Edge> = BinaryHeap::with_capacity(self.adj_list.num_edges());

        // the shortest distance of certain vertex has already been found or not
        let mut visited = vec![false; self.adj_list.num_vertices()+1];
        // the shortest distance from origin vertex to another
        // here we use -1 indicate infinity
        // todo
        // note that i32 may be exceed
        let mut distance: Vec<i32> = vec![-1; self.adj_list.num_vertices()+1];

        // initialize with origin vertex
        // boundary check may affect performance
        visited[vertex] = true;
        distance[vertex] = 0;
        
        for edge in self.adj_list.edge_iterator_of_vertex(vertex) {
            distance[edge.out_vertex] = edge.weight;
            heap.push(Edge{vertex: edge.out_vertex, weight: edge.weight});
        }

        while let Some(Edge{vertex, weight}) = heap.pop() {
            if visited[vertex] {continue;}
            visited[vertex] = true;
            distance[vertex] = weight;

            for edge in self.adj_list.edge_iterator_of_vertex(vertex) {
                if visited[edge.out_vertex] {continue;}
                
                let new_weight = weight + edge.weight;

                match distance[edge.out_vertex] {
                    -1 => {
                        heap.push(Edge{vertex: edge.out_vertex, weight: new_weight});
                        distance[edge.out_vertex] = new_weight;
                    },
                    x => {
                        
                        if new_weight < x {
                            heap.push(Edge{vertex: edge.out_vertex, weight: new_weight});
                            distance[edge.out_vertex] = new_weight;
                        }
                    }
                }
            }
        }

        println!("{:?}", distance);
        println!("{:?}", visited);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRAPH_FILE: &str = "src/graph/examples/test.txt";
    const POSITIVE_TEST_GRAPH_FILE: &str = "src/graph/examples/positive_test.txt";

    #[test]
    fn test_graph_dfs() {
        let graph = Graph::create_from_file(TEST_GRAPH_FILE);
        graph.dfs();
    }
    
    #[test]
    fn test_graph_bfs() {
        let graph = Graph::create_from_file(TEST_GRAPH_FILE);
        graph.bfs();
    }

    #[test]
    fn test_graph_dijkstra() {
        let graph = Graph::create_from_file(POSITIVE_TEST_GRAPH_FILE);
        graph.dijkstra(4);
    }
}