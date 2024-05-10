fn degree_to_rad(d: f32) -> f32 {
    d * PI / 180.0
}

fn rad_to_degree(r: f32) -> f32 {
    r * 180.0 / PI
}

#[derive(PartialEq)]
struct PointI {
    x: i32,
    y: i32,
}

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



fn not_a_triangle(a: &Point, b: &Point, c: &Point) -> bool {
    a == b || a == c || b == c || collinear(a, b, c)
}



//! Calculate angle in radian based on the length of 3 side
fn get_degree(x: f32, y: f32, z: f32) -> f32 {
    ((x.powi(2) + y.powi(2) - z.powi(2)) / (2.0 * x * y)).acos()
}

fn approx_equal(x: f32, y: f32) -> bool {
    (x - y).abs() < EPS
}

impl MathVec {
    fn cross(&self other: &MathVec) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

fn cross(a: &MathVec, b: &MathVec) -> f32 {
    a.x * b.y - a.y * b.x
}

fn collinear(a: &Point, b: &Point, c: &Point) -> bool {
    cross(&MathVec::new(a, b), &MathVec::new(a, c)).abs() < EPSILON
}




struct Line {
    a: f32,
    b: f32,
    c: f32
}




impl Line {
    fn are_parallel(&self, other: &Line) -> bool {
        return approx_equal(self.a, other.a) && approx_equal(self.b, other.b)
    }

    fn are_the_same(&self, other: &Line) -> bool {
        return are_parallel(self, other) && approx_equal(self.c, other.c)
    }

    fn intersect(&self, other: &Line) -> Option<Point> {
        if are_parallel(self, other) {
            return None;
        }
        let x  = (other.b * self.c - self.b * other.c) / (other.a & self.b - self.a * other.b);
        let y = {
            if self.b.abs() > EPS {
                return -(self.a * x + self.c);
            }
            -(other.a * x + other.c)
        }
        Some(Point{x, y})
    }
}

impl MathVec {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }

    fn scale(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s
        }
    }
}


impl Point {
    fn eucl_dist(&self, other: &Point) -> f32 {
        (self.x-other.x).hypot(self.y-other.y)    
    }

    fn rorate(&self, theta: f32) -> Self {
        let rad = degree_to_rad(theta);
        Self {
            x: p.x*rad.cos() - p.y*rad.sin(),
            y: p.x*rad.sin() + p.y*rad.cos()
        }
    }

    fn get_line(&self, other: &Point) -> Line {
        if approx_equal(self.x, other.x) {
            return Self {
                a: 1.0,
                b: 0.0,
                c: -self.x
            }
        }
        let a = -(self.y - other.y) / (self.x - other.x);
        return Self {
            a,
            b: 1.0,
            c: -a * self.x - self.y
        }
    }

    fn translate(&self, v: &MathVec) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y
        }
    }

    fn with_slope(&self, slope: f32) -> Line {
        Line{
            a: -m,
            b: 1.,
            c: -((-m * self.x) + (1.* self.y))
        }
    }
}  

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

fn intersects_sphere(center: &Vec3, radius: f64, origin: &Vec3, direction: &Vec3) -> bool {
    let oc = Vec3 {
        x: origin.x - center.x,
        y: origin.y - center.y,
        z: origin.z - center.z,
    };
    let a = direction.dot(direction);
    let b = 2.0f64 * direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return false;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);

    if t1 < 0.0 && t2 < 0.0 {
        return false;
    }
    true
}


// void closestPoint(line l, point p, point &ans) {
// // this line is perpendicular to l and pass through p
// line perpendicular;                            
// if (fabs(l.b) < EPS) {                         // vertical line
//     ans.x = -(l.c);
//     ans.y = p.y;
//     return;
// }
// if (fabs(l.a) < EPS) {                         // horizontal line
//     ans.x = p.x;
//     ans.y = -(l.c);
//     return;
// }
// pointSlopeToLine(p, 1/l.a, perpendicular);     // normal line
// // intersect line l with this perpendicular line
// // the intersection point is the closest point
// areIntersect(l, perpendicular, ans);
// }

// // returns the reflection of point on a line
// void reflectionPoint(line l, point p, point &ans) {
// point b;
// closestPoint(l, p, b);                         // similar to distToLine
// vec v = toVec(p, b);                           // create a vector
// ans = translate(translate(p, v), v);           // translate p twice
// }

// // returns the dot product of two vectors a and b
// double dot(vec a, vec b) { return (a.x*b.x + a.y*b.y); }

// // returns the squared value of the normalized vector
// double norm_sq(vec v) { return v.x*v.x + v.y*v.y; }

// double angle(const point &a, const point &o, const point &b) {
// vec oa = toVec(o, a), ob = toVec(o, b);        // a != o != b
// return acos(dot(oa, ob) / sqrt(norm_sq(oa) * norm_sq(ob)));
// }                                                // angle aob in rad

// // returns the distance from p to the line defined by
// // two points a and b (a and b must be different)
// // the closest point is stored in the 4th parameter (byref)
// double distToLine(point p, point a, point b, point &c) {
// vec ap = toVec(a, p), ab = toVec(a, b);
// double u = dot(ap, ab) / norm_sq(ab);
// // formula: c = a + u*ab
// c = translate(a, scale(ab, u));                // translate a to c
// return dist(p, c);                             // Euclidean distance
// }

// // returns the distance from p to the line segment ab defined by
// // two points a and b (technically, a has to be different than b)
// // the closest point is stored in the 4th parameter (byref)
// double distToLineSegment(point p, point a, point b, point &c) {
// vec ap = toVec(a, p), ab = toVec(a, b);
// double u = dot(ap, ab) / norm_sq(ab);
// if (u < 0.0) {                                 // closer to a
//     c = point(a.x, a.y);
//     return dist(p, a);                           // dist p to a
// }
// if (u > 1.0) {                                 // closer to b
//     c = point(b.x, b.y);
//     return dist(p, b);                           // dist p to b
// }
// return distToLine(p, a, b, c);                 // use distToLine
// }

// // returns the cross product of two vectors a and b
// double cross(vec a, vec b) { return a.x*b.y - a.y*b.x; }

// //// another variant
// // returns 'twice' the area of this triangle A-B-c
// // int area2(point p, point q, point r) {
// //   return p.x * q.y - p.y * q.x +
// //          q.x * r.y - q.y * r.x +
// //          r.x * p.y - r.y * p.x;
// // }

// // note: to accept collinear points, we have to change the `> 0'
// // returns true if point r is on the left side of line pq
// bool ccw(point p, point q, point r) {
// return cross(toVec(p, q), toVec(p, r)) > -EPS;
// }