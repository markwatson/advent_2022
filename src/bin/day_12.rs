use std::collections::{hash_map, HashMap};
use std::fmt::{Display, Formatter, Result};
use std::fs::read_to_string;

use petgraph::algo::{astar, dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Directed, Graph};

use euler::matrix::Matrix;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct MatrixIdx {
    r: usize,
    c: usize,
}

impl MatrixIdx {
    fn new(r: usize, c: usize) -> Self {
        MatrixIdx { r, c }
    }

    fn empty() -> Self {
        MatrixIdx { r: 0, c: 0 }
    }
}

fn to_edges(
    matrix: &Matrix<char>,
) -> (
    Vec<MatrixIdx>,
    Vec<(MatrixIdx, MatrixIdx)>,
    MatrixIdx,
    MatrixIdx,
    Vec<MatrixIdx>,
) {
    let mut edges = vec![];
    let mut nodes = vec![];
    let mut start_idx = MatrixIdx::empty();
    let mut end_idx = MatrixIdx::empty();
    let mut all_starts = vec![];
    for r in 0..matrix.len_rows() {
        for c in 0..matrix.len_cols() {
            let neighbors = matrix.neighbors_idx(r, c);
            nodes.push(MatrixIdx::new(r, c));
            for n in neighbors {
                let mut current = matrix.data[r][c] as i32;
                let mut neighbor = matrix.data[n.0][n.1] as i32;

                if current == 'a' as i32 {
                    all_starts.push(MatrixIdx::new(r, c));
                }

                if current == 'S' as i32 {
                    start_idx = MatrixIdx::new(r, c);
                    current = 'a' as i32;
                } else if current == 'E' as i32 {
                    end_idx = MatrixIdx::new(r, c);
                    current = 'z' as i32;
                } else if neighbor == 'E' as i32 {
                    neighbor = 'z' as i32;
                } else if neighbor == 'S' as i32 {
                    neighbor = 'a' as i32;
                }

                if neighbor <= current + 1 {
                    edges.push((MatrixIdx::new(r, c), MatrixIdx::new(n.0, n.1)));
                }
            }
        }
    }
    return (nodes, edges, start_idx, end_idx, all_starts);
}

fn build_graph(
    matrix: &Matrix<char>,
) -> (Graph<MatrixIdx, ()>, NodeIndex, NodeIndex, Vec<NodeIndex>) {
    let (nodes, edges, start_idx, end_idx, all_starts) = to_edges(matrix);

    let mut graph = Graph::new();

    let mut node_index: HashMap<MatrixIdx, NodeIndex> = HashMap::new();
    for node in nodes {
        let graph_node = graph.add_node(node.clone());
        node_index.insert(node, graph_node);
    }
    for edge in edges {
        // Flipped for part 2
        graph.update_edge(node_index[&edge.0], node_index[&edge.1], ());
    }

    let mut all_start_nodes = vec![];
    for node in all_starts {
        all_start_nodes.push(node_index[&node]);
    }

    return (
        graph,
        node_index[&start_idx],
        node_index[&end_idx],
        all_start_nodes,
    );
}

fn main() {
    // Create an undirected graph with `i32` nodes and edges with `()` associated data.
    let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
    let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
    assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());

    // Get the minimum spanning tree of the graph as a new graph, and check that
    // one edge was trimmed.
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
    assert_eq!(g.raw_edges().len() - 1, mst.raw_edges().len());

    // Output the tree to `graphviz` `DOT` format
    println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
    // graph {
    //     0 [label="\"0\""]
    //     1 [label="\"0\""]
    //     2 [label="\"0\""]
    //     3 [label="\"0\""]
    //     1 -- 2
    //     3 -- 4
    //     2 -- 3
    // }
    //cargo run --bin day_12 | dot -Tjpg > /tmp/output.jpg && open /tmp/output.jpg

    let matrix = Matrix::from_file("./data/day_12");

    for r in 0..matrix.len_rows() {
        for c in 0..matrix.len_cols() {
            let empty = ' ';
            println!("Char: {}", matrix.data[r][c]);
            println!(
                "{} {} {} {}",
                matrix.up(r, c).unwrap_or(empty),
                matrix.left(r, c).unwrap_or(empty),
                matrix.right(r, c).unwrap_or(empty),
                matrix.down(r, c).unwrap_or(empty)
            );

            println!();
        }
        println!();
    }

    println!("{}", matrix);

    let (graph, start, end, all_starts) = build_graph(&matrix);

    let costs = dijkstra(&graph, start, Some(end), |_| 1);
    println!(
        "Part 1: Found: {}",
        &costs.get(&end).expect("No path found")
    );

    let mut min = i32::MAX;
    for start in all_starts {
        let costs = dijkstra(&graph, start, Some(end), |_| 1);
        if !costs.contains_key(&end) {
            continue;
        }
        let the_cost = costs.get(&end).unwrap();
        if the_cost < &min {
            min = *the_cost;
        }
    }
    println!("Part 2: Found: {}", min);
}
