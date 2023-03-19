fn manhattan_distance(a: &Node, b: &Node) -> f64 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    dx + dy
}

fn euclidean_distance(a: &Node, b: &Node) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx.powi(2) + dy.powi(2)).sqrt()
}
