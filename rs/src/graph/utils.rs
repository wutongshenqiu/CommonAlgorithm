use rand;
use rand::seq::SliceRandom;
use rand::Rng;

use std::fs::File;
use std::io::Write;

use crate::graph::Edge;

struct FakeTool {
    num_vertices: usize,
    num_edges: usize,
    weight_range: (i32, i32),
}

impl FakeTool {
    fn gen_vertices(&self) -> Vec<usize> {
        let mut vertices = Vec::with_capacity(self.num_vertices);
        for i in 1..(self.num_vertices + 1) {
            vertices.push(i);
        }

        vertices
    }

    pub fn generate(&self) -> Vec<Edge> {
        let mut rng = rand::thread_rng();

        // store choosed edge
        let mut edges: Vec<Edge> = Vec::with_capacity(self.num_edges);

        // get number of edge for each vertex
        let num_edges =
            gen_random_integers(self.num_vertices, self.num_edges, (1, self.num_vertices));
        // println!("{:?}", num_edges);

        for (in_vertex, num_edge) in num_edges.iter().enumerate() {
            let in_vertex = in_vertex + 1;

            // println!("in vertex: {}", in_vertex);

            let mut out_vertices: Vec<usize> = Vec::with_capacity(self.num_vertices);
            for vertex in 1..(self.num_vertices + 1) {
                match vertex {
                    _ if vertex != in_vertex => out_vertices.push(vertex),
                    _ => continue,
                }
            }

            out_vertices.shuffle(&mut rng);
            // println!("out vertices: {:?}", out_vertices);

            let mut num = 0;
            for out_vertex in out_vertices.into_iter() {
                let (lower_bound, upper_bound) = self.weight_range;
                let weight: i32 = rng.gen_range(lower_bound, upper_bound);
                edges.push(Edge {
                    in_vertex,
                    out_vertex,
                    weight,
                });

                num += 1;

                if num >= *num_edge {
                    break;
                }
            }
        }
        edges
    }

    pub fn write_to_file(&self, edges: Vec<Edge>, file_name: &str) {
        let mut file = File::create(file_name).unwrap();

        writeln!(file, "{} {}", self.num_vertices, self.num_edges).unwrap();

        for edge in edges {
            writeln!(
                file,
                "{} {} {}",
                edge.in_vertex, edge.out_vertex, edge.weight
            )
            .unwrap();
        }
    }
}

/// builder pattern
/// https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md
/// https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
/// https://www.reddit.com/r/rust/comments/ak4h2l/the_builder_pattern_and_functional_programming/

struct FakeToolBuilder {
    num_vertices: usize,
    num_edges: usize,
    weight_range: (i32, i32),
}

impl FakeToolBuilder {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let num_vertices: usize = rng.gen_range(5, 20);
        let num_edges: usize =
            rng.gen_range(num_vertices + 1, num_vertices * (num_vertices - 1) / 2);

        FakeToolBuilder {
            num_vertices,
            num_edges,
            weight_range: (-10, 10),
        }
    }

    pub fn set_num_vertices(&mut self, num_vertices: usize) -> &mut Self {
        self.num_vertices = num_vertices;
        self
    }

    pub fn set_num_edges(&mut self, num_edges: usize) -> &mut Self {
        self.num_edges = num_edges;
        self
    }

    pub fn set_weight_range(mut self, weight_range: (i32, i32)) -> Self {
        self.weight_range = weight_range;
        self
    }

    /// we can establish template if we not consume FakeToolBuilder here
    pub fn finish(&self) -> FakeTool {
        FakeTool {
            num_vertices: self.num_vertices,
            num_edges: self.num_edges,
            weight_range: self.weight_range,
        }
    }
}

/// generate m random positive integers range to m_range that sum to n
/// this is not a standard method
pub fn gen_random_integers(m: usize, n: usize, m_range: (usize, usize)) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    if m > n {
        panic!("n({}) must less or equal to m({})!", m, n);
    }

    if n > m * m_range.1 {
        panic!(
            "m({}) * {} must greater or equal than n({})",
            m, m_range.1, n
        );
    }

    let mut random_integers: Vec<usize> = Vec::with_capacity(m);
    let mut current_sum: usize = 0;

    for i in (1..m).rev() {
        let lower_bound = {
            match m_range.0 {
                x if x > 1 => x,
                _ => 1,
            }
        };

        let upper_bound = {
            match m_range.1 {
                x if x < n - current_sum - i + 1 => x,
                _ => n - current_sum - i + 1,
            }
        };

        let random_integer = rng.gen_range(lower_bound, upper_bound);
        current_sum += random_integer;
        random_integers.push(random_integer);
    }
    random_integers.push(n - current_sum);

    random_integers.shuffle(&mut rng);

    random_integers
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRAPH_DIR: &str = "src/graph/examples";
    const SIMPLE_GRAPH: (usize, usize) = (1000, 50000);
    const MEDIUM_GRAPH: (usize, usize) = (10000, 1000000);
    const COMPLICATED_GRAPH: (usize, usize) = (30000, 5000000);
    const DENSE_GRAPH: (usize, usize) = (10000, 20000000);
    const WEIGHT_RANGE: (i32, i32) = (-100, 200);
    const POSITIVE_WEIGHT_RANGE: (i32, i32) = (1, 100);

    fn generate_positive_fake_graph_file() {
        let mut faketool_builder = FakeToolBuilder::new().set_weight_range(POSITIVE_WEIGHT_RANGE);

        let simple_graph_faketool = faketool_builder
            .set_num_vertices(SIMPLE_GRAPH.0)
            .set_num_edges(SIMPLE_GRAPH.1)
            .finish();

        let edges = simple_graph_faketool.generate();
        simple_graph_faketool
            .write_to_file(edges, &format!("{}/{}.txt", GRAPH_DIR, "positive_simple"));

        let medium_graph_faketool = faketool_builder
            .set_num_vertices(MEDIUM_GRAPH.0)
            .set_num_edges(MEDIUM_GRAPH.1)
            .finish();

        let edges = medium_graph_faketool.generate();
        medium_graph_faketool
            .write_to_file(edges, &format!("{}/{}.txt", GRAPH_DIR, "positive_medium"));

        let complicated_graph_faketool = faketool_builder
            .set_num_vertices(COMPLICATED_GRAPH.0)
            .set_num_edges(COMPLICATED_GRAPH.1)
            .finish();

        let edges = complicated_graph_faketool.generate();
        complicated_graph_faketool.write_to_file(
            edges,
            &format!("{}/{}.txt", GRAPH_DIR, "positive_complicated"),
        );

        // let dense_graph_faketool = faketool_builder
        //     .set_num_vertices(DENSE_GRAPH.0)
        //     .set_num_edges(DENSE_GRAPH.1)
        //     .finish();

        // let edges = dense_graph_faketool.generate();
        // dense_graph_faketool.write_to_file(
        //     edges,
        //     &format!("{}/{}.txt", GRAPH_DIR, "positive_dense"),
        // );    
    }

    fn generate_fake_graph_file() {
        let mut faketool_builder = FakeToolBuilder::new().set_weight_range(WEIGHT_RANGE);

        let simple_graph_faketool = faketool_builder
            .set_num_vertices(SIMPLE_GRAPH.0)
            .set_num_edges(SIMPLE_GRAPH.1)
            .finish();

        let edges = simple_graph_faketool.generate();
        simple_graph_faketool.write_to_file(edges, &format!("{}/{}.txt", GRAPH_DIR, "simple"));

        let medium_graph_faketool = faketool_builder
            .set_num_vertices(MEDIUM_GRAPH.0)
            .set_num_edges(MEDIUM_GRAPH.1)
            .finish();

        let edges = medium_graph_faketool.generate();
        medium_graph_faketool.write_to_file(edges, &format!("{}/{}.txt", GRAPH_DIR, "medium"));

        let complicated_graph_faketool = faketool_builder
            .set_num_vertices(COMPLICATED_GRAPH.0)
            .set_num_edges(COMPLICATED_GRAPH.1)
            .finish();

        let edges = complicated_graph_faketool.generate();
        complicated_graph_faketool
            .write_to_file(edges, &format!("{}/{}.txt", GRAPH_DIR, "complicated"));
    }

    fn test_gen_random_integers() {
        assert_eq!(vec![1, 1, 1, 1, 1], gen_random_integers(5, 5, (1, 100)));
        assert_eq!(vec![1, 1, 1], gen_random_integers(3, 3, (1, 100)));

        let test_sum1: usize = gen_random_integers(15, 100, (1, 15)).iter().sum();
        assert_eq!(100, test_sum1);
    }
}
