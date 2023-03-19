fn manhattan_distance(current: &Node, goal: &Node) -> f64 {
    let dx = (current.x - goal.x).abs();
    let dy = (current.y - goal.y).abs();
    dx + dy
}

fn euclidean_distance(current: &Node, goal: &Node) -> f64 {
    let dx = (current.x - goal.x) as f64;
    let dy = (current.y - goal.y) as f64;
    (dx.powi(2) + dy.powi(2)).sqrt()
}
