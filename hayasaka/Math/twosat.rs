mod twosat {
    pub struct TwoSat {
        n: usize,
        adj: Vec<Vec<usize>>,
        adj_t: Vec<Vec<usize>>,
    }

    impl TwoSat {
        pub fn new(n: usize) -> Self {
            let n = n << 2;
            Self {
                n,
                adj: vec![vec![]; n],
                adj_t: vec![vec![]; n],
            }
        }

        // na and nb signify whether a and b are to be negated
        pub fn add_disjunction(&mut self, a: usize, na: bool, b: usize, nb: bool) {
            let a = (a << 1) ^ na as usize;
            let b = (b << 1) ^ nb as usize;
            let neg_a = a ^ 1;
            let neg_b = b ^ 1;
            self.adj[neg_a].push(b);
            self.adj[neg_b].push(a);
            self.adj_t[b].push(neg_a);
            self.adj_t[a].push(neg_b);
        }

        pub fn solve(self) -> (bool, Vec<bool>) {
            let mut order = vec![];
            let n = self.n;
            let mut comp = vec![-1; n];
            let mut used = vec![false; n];
            let adj = self.adj;
            let adj_t = self.adj_t;

            fn dfs1(v: usize, used: &mut [bool], adj: &[Vec<usize>], order: &mut Vec<usize>) {
                used[v] = true;
                for &u in &adj[v] {
                    if !used[u] {
                        dfs1(u, used, adj, order);
                    }
                }
                order.push(v);
            }
            fn dfs2(v: usize, cl: i32, comp: &mut [i32], adj_t: &[Vec<usize>]) {
                comp[v] = cl;
                for &u in &adj_t[v] {
                    if comp[u] == -1 {
                        dfs2(u, cl, comp, adj_t);
                    }
                }
            }
            for i in 0..n {
                if !used[i] {
                    dfs1(i, &mut used, &adj, &mut order);
                }
            }
            let mut j = 0;
            for i in 0..n {
                let v = order[self.n - i - 1];
                if comp[v] == -1 {
                    dfs2(v, j, &mut comp, &adj_t);
                    j += 1;
                }
            }
            let mut assignment = vec![false; n / 2];
            for i in (0..n).step_by(2) {
                if comp[i] == comp[i + 1] {
                    return (false, assignment);
                }
                assignment[i / 2] = comp[i] > comp[i + 1];
            }
            (true, assignment)
        }
    }
}
