// conex hull trick to speed up DP
// https://blog.hamayanhamayan.com/entry/2019/01/12/161237
mod hayasaka {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    type Coef = i64;
    type Coord = i64;
    type Val = i64;

    #[derive(Debug)]
    pub struct ConvexHullDynamic {
        hull: VecDeque<Line>, // Envelope itself
        is_max: bool,         // Whether the saved envelope is for max value
    }

    #[derive(Debug, Clone, Copy)]
    struct Line {
        a: Coef,             // Coefficient 'a' of the line equation y = a*x + b
        b: Coef,             // Coefficient 'b' of the line equation y = a*x + b
        x_left: f64, // Intersection with the previous line in the hull (first line has -INF)
        line_type: LineType, // Type of the line (normal line or query for max/min)
        val: Coord,  // Value for the query (used for max/min queries)
    }

    impl PartialEq for Line {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a
                && self.b == other.b
                && self.x_left == other.x_left
                && self.line_type == other.line_type
                && self.val == other.val
        }
    }

    impl Eq for Line {}

    impl Line {
        fn new(a: Coef, b: Coef) -> Self {
            Self {
                a,
                b,
                x_left: f64::NEG_INFINITY,
                line_type: LineType::Line,
                val: 0,
            }
        }
        /// Computes the value of the line at coordinate x.
        fn value_at(&self, x: Coord) -> Val {
            self.a * x + self.b
        }
    }

    impl Default for Line {
        fn default() -> Self {
            Self::new(0, 0)
        }
    }

    /// Checks if two lines are parallel.
    fn are_parallel(l1: &Line, l2: &Line) -> bool {
        l1.a == l2.a
    }

    /// Computes the x-coordinate of intersection between two lines.
    fn intersect_x(l1: &Line, l2: &Line) -> f64 {
        if are_parallel(l1, l2) {
            std::f64::INFINITY
        } else {
            (l2.b - l1.b) as f64 / (l1.a - l2.a) as f64
        }
    }

    impl PartialOrd for Line {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Line {
        fn cmp(&self, other: &Self) -> Ordering {
            match other.line_type {
                LineType::Line => self.a.cmp(&other.a),
                LineType::MaxQuery => self.x_left.partial_cmp(&(other.val as f64)).unwrap(),
                LineType::MinQuery => (other.val as f64).partial_cmp(&self.x_left).unwrap(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LineType {
        Line,     // Normal line
        MaxQuery, // Query for maximum value
        MinQuery, // Query for minimum value
    }

    impl ConvexHullDynamic {
        /// Constructs a new ConvexHullDynamic instance.
        pub fn new(is_max: bool) -> Self {
            ConvexHullDynamic {
                hull: VecDeque::new(),
                is_max,
            }
        }

        /// Checks if there is a previous line in the hull.
        fn has_prev(&self, it: usize) -> bool {
            it > 0
        }

        /// Checks if there is a next line in the hull.
        fn has_next(&self, it: usize) -> bool {
            it + 1 < self.hull.len()
        }

        /// Checks if line `l2` is irrelevant between `l1` and `l3`.
        fn irrelevant(&self, l1: &Line, l2: &Line, l3: &Line) -> bool {
            intersect_x(l1, l3) <= intersect_x(l1, l2)
        }

        fn irrelevant_it(&self, it: usize) -> bool {
            self.has_prev(it)
                && self.has_next(it)
                && (self.is_max
                    && self.irrelevant(&self.hull[it - 1], &self.hull[it], &self.hull[it + 1])
                    || !self.is_max
                        && self.irrelevant(&self.hull[it + 1], &self.hull[it], &self.hull[it - 1]))
        }

        /// Updates the `x_left` of the line at index `idx`.
        fn update_left_border(&mut self, idx: usize) {
            if (self.is_max && !self.has_prev(idx)) || (!self.is_max && !self.has_next(idx)) {
                return;
            }

            let val = intersect_x(
                &self.hull[idx],
                if self.is_max {
                    &self.hull[idx - 1]
                } else {
                    &self.hull[idx + 1]
                },
            );
            self.hull[idx].x_left = val;
        }

        /// Adds a line y = ax + b to the hull.
        pub fn add_line(&mut self, a: Coef, b: Coef) {
            let l3 = Line::new(a, b);
            let it = self
                .hull
                .binary_search_by(|element| match element.cmp(&l3) {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                })
                .unwrap_err();

            if it != self.hull.len() && are_parallel(&self.hull[it], &l3) {
                if self.is_max && self.hull[it].b < b || !self.is_max && self.hull[it].b > b {
                } else {
                    return;
                }
            }

            if it == self.hull.len() {
                self.hull.push_back(l3)
            } else {
                self.hull[it] = l3;
            }

            if self.irrelevant_it(it) {
                self.hull.remove(it);
                return;
            }

            let mut prunes = HashSet::new();
            let mut cit = it;
            while self.has_prev(cit) && self.irrelevant_it(cit - 1) {
                prunes.insert(cit - 1);
                cit -= 1;
            }

            let mut cit = it;
            while self.has_next(cit) && self.irrelevant_it(cit + 1) {
                prunes.insert(cit + 1);
                cit += 1;
            }
            let mut new_hull = VecDeque::new();
            for i in 0..self.hull.len() {
                if prunes.contains(&i) {
                    continue;
                }
                new_hull.push_back(self.hull[i].clone());
            }
            std::mem::swap(&mut self.hull, &mut new_hull);

            self.update_left_border(it);

            if self.has_prev(it) {
                self.update_left_border(it - 1);
            }
            if self.has_next(it) {
                self.update_left_border(it + 1);
            }
        }

        /// Gets the best value at x according to the current hull.
        pub fn get_best(&self, x: Coord) -> Val {
            let q = Line {
                a: 0,
                b: 0,
                x_left: 0.0,
                line_type: if self.is_max {
                    LineType::MaxQuery
                } else {
                    LineType::MinQuery
                },
                val: x,
            };

            let mut best_line = self
                .hull
                .binary_search_by(|element| match element.cmp(&q) {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                })
                .unwrap_err();

            if self.is_max {
                best_line -= 1;
            }
            self.hull[best_line].value_at(x)
        }
    }
}
use hayasaka::*;

// Li-Chao tree
mod hayasaka {
    type Data = i64;

    pub struct LiChaoTree {
        query_type: QueryType,
        left: Data,
        right: Data,
        lines: Vec<(Data, Data)>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum QueryType {
        Min,
        Max,
    }

    fn get_default(query_type: QueryType) -> Data {
        use QueryType::*;
        match query_type {
            Min => Data::MAX,
            Max => Data::MIN,
        }
    }

    impl LiChaoTree {
        /// Creates a new tree, built to handle queries on the interval [left, right).
        pub fn new(left: Data, right: Data, query_type: QueryType) -> Self {
            Self {
                query_type,
                left,
                right,
                lines: vec![(0, get_default(query_type)); (right - left) as usize],
            }
        }

        /// Every node in the tree has the property that the line that maximizes its midpoint is found
        /// either in the node or one of its ancestors.  When we visit a node, we compute the winner at
        /// the midpoint of the node. The winner is stored in the node. The loser can still possibly
        /// beat the winner on some segment, either to the left or to the right of the current
        /// midpoint, so we propagate it to that segment. This sequence ensures that the invariant is
        /// kept.
        fn add_line_impl(&mut self, mut m: Data, mut b: Data, l: Data, r: Data) {
            if r <= l {
                return;
            }
            let ix = ((r - self.left + l - self.left) / 2) as usize;
            let mid = self.left + (ix as Data);
            let (ref mut m_ix, ref mut b_ix) = self.lines[ix];
            use QueryType::*;
            match self.query_type {
                Min => {
                    if m * mid + b < *m_ix * mid + *b_ix {
                        std::mem::swap(&mut m, m_ix);
                        std::mem::swap(&mut b, b_ix);
                    }
                    if m > *m_ix {
                        self.add_line_impl(m, b, l, mid);
                    } else if m < *m_ix {
                        self.add_line_impl(m, b, mid + 1, r);
                    }
                }
                Max => {
                    if m * mid + b > *m_ix * mid + *b_ix {
                        std::mem::swap(&mut m, m_ix);
                        std::mem::swap(&mut b, b_ix);
                    }
                    if m < *m_ix {
                        self.add_line_impl(m, b, l, mid);
                    } else if m > *m_ix {
                        self.add_line_impl(m, b, mid + 1, r);
                    }
                }
            }
        }

        /// Adds the line with slope m and intercept b. O(log N) complexity.
        pub fn add_line(&mut self, m: Data, b: Data) {
            self.add_line_impl(m, b, self.left, self.right);
        }

        /// Because of the invariant established by add_line, we know that the best line for a given
        /// point is stored in one of the ancestors of its node. So we accumulate the maximum answer as
        /// we go back up the tree.
        fn get_impl(&self, x: Data, l: Data, r: Data) -> Data {
            if r == l {
                return get_default(self.query_type);
            }
            let ix = ((r - self.left + l - self.left) / 2) as usize;
            let mid = ix as i64 + self.left;
            let y = self.lines[ix].0 * x + self.lines[ix].1;
            use QueryType::*;
            let fnc = {
                match self.query_type {
                    Min => |&x: &Data, &y: &Data| x.min(y),
                    Max => |&x: &Data, &y: &Data| x.max(y),
                }
            };
            if x == mid {
                y
            } else if x < mid {
                fnc(&self.get_impl(x, l, mid), &y)
            } else {
                fnc(&self.get_impl(x, mid + 1, r), &y)
            }
        }

        /// Finds the maximum mx+b among all lines in the structure. O(log N) complexity.
        pub fn get(&self, x: i64) -> i64 {
            self.get_impl(x, self.left, self.right)
        }
    }
}
use hayasaka::*;
