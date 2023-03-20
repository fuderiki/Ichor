use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};



#[derive(Debug, Clone)]
pub struct Edge {
    target: usize,
    cost: i32,
}

pub type Graph = HashMap<usize, Vec<Edge>>;

fn astar_search(
    graph: &Graph,
    nodes: &HashMap<usize, Node>,
    start: usize,
    goal: usize,
    h: fn(&Node, &Node) -> f64, ) -> Option<Vec<usize>> {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NodeWrapper {
        id: usize,
        f_cost: f64,
        g_cost: i32,
    }

    impl Ord for NodeWrapper {
        fn cmp(&self, other: &Self) -> Ordering {
            other.f_cost.partial_cmp(&self.f_cost).unwrap_or(Ordering::Equal)
        }
    }

    impl PartialOrd for NodeWrapper {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_costs = HashMap::new();

    open_set.push(NodeWrapper {
        id: start,
        f_cost: 0.0,
        g_cost: 0,
    });
    g_costs.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.id == goal {
            let mut path = Vec::new();
            let mut current_id = goal;

            while let Some(prev_id) = came_from.remove(&current_id) {
                path.push(current_id);
                current_id = prev_id;
            }

            path.push(start);
            path.reverse();
            return Some(path);
        }

        for edge in graph.get(&current.id).unwrap_or(&Vec::new()) {
            let tentative_g_cost = current.g_cost + edge.cost;
            let g_cost_entry = g_costs.entry(edge.target).or_insert(i32::MAX);

            if tentative_g_cost < *g_cost_entry {
                *g_cost_entry = tentative_g_cost;
                came_from.insert(edge.target, current.id);

                let h_cost = h(&nodes[&edge.target], &nodes[&goal]);
                open_set.push(NodeWrapper {
                    id: edge.target,
                    f_cost: tentative_g_cost as f64 + h_cost,
                    g_cost: tentative_g_cost,
                });
            }
        }
    }

    None
}


pub fn astar_cost_function(current: &Node, goal: &Node, g: i32, h: fn(&Node, &Node) -> f64) -> f64 {
    let current_cost = g;
    let heuristic_cost = h(current, goal); //Distance Function
    actual_cost + heuristic_cost
}


