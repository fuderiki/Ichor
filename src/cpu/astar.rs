pub fn astar_cost_function(current: &Node, goal: &Node, g: i32, h: fn(&Node, &Node) -> i32) -> i32 {
    let current_cost = g;
    let heuristic_cost = h(current, goal); //Distance Function
    actual_cost + heuristic_cost
}
