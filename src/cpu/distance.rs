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

/*faster to compute euclidean distance if the square root is ommitted*/ 
fn euclidean_squared_distance(a: &Node, b: &Node) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    dx.powi(2) + dy.powi(2)
}

fn chebyshev_distance(a: &Node, b: &Node) -> f64 {
    let dx = (a.x - b.x).abs() as f64;
    let dy = (a.y - b.y).abs() as f64;
    f64::max(dx, dy)
}

fn octile_distance(a: &Node, b: &Node) -> f64 {
    const D1: f64 = 1.0; // Cost of moving horizontally/vertically
    const D2: f64 = (2.0 as f64).sqrt(); // Cost of moving diagonally

    let dx = (a.x - b.x).abs() as f64;
    let dy = (a.y - b.y).abs() as f64;

    D1 * (dx + dy) + (D2 - 2.0 * D1) * f64::min(dx, dy)
}

