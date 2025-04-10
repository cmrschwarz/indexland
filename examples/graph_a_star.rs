//! NOTE: this is a practical example. If you're looking for more of an
//! api listing check out iteration.rs instead.
//!
//! This implements a basic A* graph example.
//!
//! Nothing special, but it demonstrates the use of [`IndexVec`] to
//! easily and ergonomically work with graphs,
//! which is typically considered challenging without libraries
//! due the borrow checker preventing cyclic references.
//!
//! The implementation details are mostly irrelevant, this is just meant to
//! show a few indexland features, each highlighted with `// NOTE`.

use indexland::{index_vec, Idx, IndexSlice, IndexVec, NonMax};
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Idx)]
pub struct NodeId(u32);

// NOTE: Using [`NonMax<u32>`] ensures that [`Option<EdgeId>`] is
// 4 bytes instead of 8, which saves some space in the A* `came_from` table.
#[derive(Idx)]
pub struct EdgeId(NonMax<u32>);

#[derive(Default, Clone, Debug)]
pub struct Graph {
    // NOTE: specifiying the Id types here removes any uncertanty a
    // reader might have if we had just used `Vec<Node>`.
    nodes: IndexVec<NodeId, Node>,
    edges: IndexVec<EdgeId, Edge>,
}

// position in space
#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    // NOTE: no point in using an `IndexVec` here as we don't care about
    // the ordering of these edges. In a codebase using `IndexVec` and friends
    // consistently, this also immediately gives the reader additonal information.
    edges: Vec<EdgeId>,
    pos: Position,
}

#[derive(Clone, Debug)]
pub struct Edge {
    from: NodeId,
    to: NodeId,
    cost: i32,
}

// A* Node metadata
#[derive(PartialEq, Eq)]
struct NodeInfo {
    node: NodeId,
    cost: i32,
    distance_to_goal_estimate: i32,
}

// Custom ordering for our priority queue
impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.distance_to_goal_estimate)
            .cmp(&(self.cost + self.distance_to_goal_estimate))
            .then_with(|| {
                other
                    .distance_to_goal_estimate
                    .cmp(&self.distance_to_goal_estimate)
            })
    }
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub const fn new() -> Self {
        Self {
            // NOTE: all indexland containers have zero alloc const new.
            // (Caveat for HashMap Hashers explained in the doc.)
            nodes: IndexVec::new(),
            edges: IndexVec::new(),
        }
    }

    pub fn add_node(
        &mut self,
        name: impl Into<String>,
        x: i32,
        y: i32,
    ) -> NodeId {
        // NOTE: `push_get_idx` is a convenience helper added by `indexland`.
        // There's quite a few more of these to help make working with typed
        // indices as pleasant as possible.
        self.nodes.push_get_idx(Node {
            name: name.into(),
            edges: Vec::new(),
            pos: Position { x, y },
        })
    }

    // For simplicity we just add all edges bidirectionally.
    // NOTE: newtype indices make sure we can never accidentally pass
    // the wrong type here which can lead to very helful error messages
    // e.g. when when refactoring parameter order.
    pub fn add_bidi_edge(&mut self, from: NodeId, to: NodeId, cost: i32) {
        let edge_id_fwd = self.edges.push_get_idx(Edge { from, to, cost });
        self.nodes[from].edges.push(edge_id_fwd);

        let edge_id_bwd = self.edges.push_get_idx(Edge {
            from: to,
            to: from,
            cost,
        });
        self.nodes[to].edges.push(edge_id_bwd);
    }

    fn calculate_distance(&self, from: NodeId, to: NodeId) -> i32 {
        let from = self.nodes[from].pos;
        let to = self.nodes[to].pos;
        // cost underestimation by rounding down
        (((to.x - from.x).pow(2) + (to.y - from.y).pow(2)) as f32).sqrt()
            as i32
    }

    pub fn find_path(
        &self,
        start: NodeId,
        goal: NodeId,
    ) -> Option<Vec<(NodeId, EdgeId)>> {
        let mut open_set = BinaryHeap::new();
        // NOTE: again, `IndexVec`s make this code much more self explanatory
        let mut came_from: IndexVec<NodeId, Option<EdgeId>> = index_vec![];
        let mut g_score: IndexVec<NodeId, i32> = index_vec![];

        // NOTE: `indices` is another convenience helper for iterating over the
        // typed indices of a container.
        // (In this case `0..nodes.len()` would have worked aswell).
        for _ in self.nodes.indices() {
            came_from.push(None);
            // Initialize best found route score with "infinity"
            g_score.push(i32::MAX);
        }

        g_score[start] = 0;
        open_set.push(NodeInfo {
            node: start,
            cost: 0,
            distance_to_goal_estimate: self.heuristic(start, goal),
        });

        while let Some(current) = open_set.pop() {
            if current.node == goal {
                return Some(self.reconstruct_path(&came_from, goal));
            }

            for &edge_id in &self.nodes[current.node].edges {
                let edge = &self.edges[edge_id];
                let neighbor = edge.to;
                let tentative_g_score = g_score[current.node] + edge.cost;

                if tentative_g_score < g_score[neighbor] {
                    came_from[neighbor] = Some(edge_id);
                    g_score[neighbor] = tentative_g_score;
                    open_set.push(NodeInfo {
                        node: neighbor,
                        cost: tentative_g_score,
                        distance_to_goal_estimate: self
                            .heuristic(neighbor, goal),
                    });
                }
            }
        }

        None
    }

    fn heuristic(&self, from: NodeId, to: NodeId) -> i32 {
        self.calculate_distance(from, to)
    }

    fn reconstruct_path(
        &self,
        came_from: &IndexSlice<NodeId, Option<EdgeId>>,
        goal: NodeId,
    ) -> Vec<(NodeId, EdgeId)> {
        let mut path = Vec::new();
        let mut current = goal;

        while let Some(edge) = came_from[current] {
            path.push((current, edge));
            current = self.edges[edge].from;
        }

        path.reverse();
        path
    }
}

// Simple Demo:
fn main() -> Result<(), i8> {
    let mut graph = Graph::new();

    // Example graph. Physical distance < cost estimate.
    //     0   1   2   3   4   5   6   7   8
    //     +---+---+---+---+---+---+---+---+
    // 0   |       A --2-- B               |
    //     |      / \     / \              |
    // 1   |     3   5   2   3             |
    //     |    /     \ /     \            |
    // 2   |   S---7---D---2---E---1---G   |
    //     |    \     /       /            |
    // 3   |     4   3       3             |
    //     |      \ /       /              |
    // 4   |       C---3---F               |
    //     +---+---+---+---+---+---+---+---+

    // Create a simple graph
    let s = graph.add_node("S", 1, 2);
    let a = graph.add_node("A", 2, 0);
    let b = graph.add_node("B", 4, 0);
    let c = graph.add_node("C", 2, 4);
    let d = graph.add_node("D", 3, 2);
    let e = graph.add_node("E", 5, 2);
    let f = graph.add_node("F", 4, 4);
    let g = graph.add_node("G", 7, 2);

    graph.add_bidi_edge(s, a, 3);
    graph.add_bidi_edge(s, c, 4);
    graph.add_bidi_edge(s, d, 7);
    graph.add_bidi_edge(a, b, 2);
    graph.add_bidi_edge(a, d, 5);
    graph.add_bidi_edge(b, d, 2);
    graph.add_bidi_edge(b, e, 3);
    graph.add_bidi_edge(c, d, 3);
    graph.add_bidi_edge(c, f, 3);
    graph.add_bidi_edge(d, e, 2);
    graph.add_bidi_edge(e, f, 3);
    graph.add_bidi_edge(e, g, 1);

    let Some(path) = graph.find_path(s, g) else {
        println!("No path found!");
        return Err(1);
    };

    println!("Path found! Node sequence:");

    let start = &graph.nodes[s];
    println!(
        "Node {} at position: ({}, {}) [total cost 0]",
        start.name, start.pos.x, start.pos.y
    );
    let mut total_cost = 0;
    for (node, edge) in path {
        total_cost += graph.edges[edge].cost;
        let node = &graph.nodes[node];
        println!(
            "Node {} at position: ({}, {}) [total cost {}]",
            node.name, node.pos.x, node.pos.y, total_cost
        );
    }
    Ok(())
}
