use super::AdjacencyList;
use super::load_adj_list_from_file;

use std::collections::VecDeque;



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
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRAPH_FILE: &str = "src/graph/examples/test.txt";

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
}