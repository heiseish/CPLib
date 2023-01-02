#[derive(PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq)]
struct MathVec {
    x: f32,
    y: f32,
}

impl MathVec {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }
}

fn not_a_triangle(a: &Point, b: &Point, c: &Point) -> bool {
    a == b || a == c || b == c || collinear(a, b, c)
}

fn get_side(a: &Point, b: &Point) -> f32 {
    ((a.x - b.x).abs().powi(2) + (a.y - b.y).abs().powi(2)).sqrt()
}

fn get_degree(x: f32, y: f32, z: f32) -> f32 {
    ((x.powi(2) + y.powi(2) - z.powi(2)) / (2.0 * x * y)).acos()
}

fn approx_equal(x: f32, y: f32) -> bool {
    (x - y).abs() < 1e-4
}
fn cross(a: &MathVec, b: &MathVec) -> f32 {
    a.x * b.y - a.y * b.x
}

fn collinear(a: &Point, b: &Point, c: &Point) -> bool {
    cross(&MathVec::new(a, b), &MathVec::new(a, c)).abs() < EPSILON
}
