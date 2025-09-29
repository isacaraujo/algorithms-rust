use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

type Graph = HashMap<usize, Vec<(usize, usize)>>;

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();

    for &node in graph.keys() {
        distances.insert(node, usize::MAX);
    }

    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > distances[&position] {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for &(neighbor, weight) in neighbors {
                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };

                if next.cost < distances[&neighbor] {
                    distances.insert(neighbor, next.cost);
                    heap.push(next);
                }
            }
        }
    }

    distances
}

fn create_sample_graph() -> Graph {
    static DEFAULT_WEIGHT: usize = 1;
    let mut graph = Graph::new();
    // 0  1  2  3
    // 4  5  6  7
    // 8  9  10 11
    // 12 13 14 15

    // x_len * y + x;
    // eg.
    //   4 * 0 + 3 = 3
    //   4 * 3 + 2 = 14

    graph.insert(0, vec![(1, DEFAULT_WEIGHT), (4, DEFAULT_WEIGHT), (5, DEFAULT_WEIGHT)]);
    graph.insert(1, vec![(2, DEFAULT_WEIGHT), (5, DEFAULT_WEIGHT)]);
    graph.insert(2, vec![(3, DEFAULT_WEIGHT), (6, DEFAULT_WEIGHT)]);
    graph.insert(3, vec![(7, DEFAULT_WEIGHT)]);
    graph.insert(4, vec![(5, DEFAULT_WEIGHT), (8, DEFAULT_WEIGHT)]);
    graph.insert(5, vec![(6, DEFAULT_WEIGHT), (9, DEFAULT_WEIGHT), (10, DEFAULT_WEIGHT)]);
    graph.insert(6, vec![(10, DEFAULT_WEIGHT), (7, DEFAULT_WEIGHT)]);
    graph.insert(7, vec![(11, DEFAULT_WEIGHT)]);
    graph.insert(8, vec![(9, DEFAULT_WEIGHT), (12, DEFAULT_WEIGHT)]);
    graph.insert(9, vec![(10, DEFAULT_WEIGHT), (13, DEFAULT_WEIGHT), (15, DEFAULT_WEIGHT)]);
    graph.insert(10, vec![(11, DEFAULT_WEIGHT), (14, DEFAULT_WEIGHT)]);
    graph.insert(11, vec![(15, DEFAULT_WEIGHT)]);
    graph.insert(12, vec![(13, DEFAULT_WEIGHT)]);
    graph.insert(13, vec![(14, DEFAULT_WEIGHT)]);
    graph.insert(14, vec![(15, DEFAULT_WEIGHT)]);
    graph.insert(15, vec![]);

    graph

}

fn main() {
    let start = 0;
    let graph = create_sample_graph();

    println!("Graph structure:");

    for (node, neighbors) in &graph {
        println!("Node {}: {:?}", node, neighbors);
    }

    let distances = dijkstra(&graph, start);

    println!("\n------------------\nSortest distance from node {}:", start);

    for (node, distance) in &distances {
        if *node == start {
            continue;
        } else if *distance == usize::MAX {
            println!("Node {} unreachable", node);
        } else {
            println!("Node {}: {}", node, distance);
        }
    }
}
