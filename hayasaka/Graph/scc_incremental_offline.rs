mod radecki {
    type Graph = Vec<Vec<usize>>;
    struct SccResult {
        pub num_sccs: usize,
        pub scc_id: Vec<usize>,
        pub scc_comps: Vec<Vec<usize>>,
    }

    fn scc(g: &Graph, g_rev: &Graph) -> SccResult {
        let n = g.len();
        fn dfs1(u: usize, used: &mut [bool], g: &Graph, order: &mut Vec<usize>) {
            used[u] = true;
            for &v in &g[u] {
                if !used[v] {
                    dfs1(v, used, g, order);
                }
            }
            order.push(u);
        }
        fn dfs2(u: usize, used: &mut [bool], g_rev: &Graph, comps: &mut Vec<usize>) {
            used[u] = true;
            comps.push(u);
            for &v in &g_rev[u] {
                if !used[v] {
                    dfs2(v, used, g_rev, comps);
                }
            }
        }
        let mut used = vec![false; n];
        let mut order = vec![];
        let mut res = vec![];
        for i in 0..n {
            if !used[i] {
                dfs1(i, &mut used, g, &mut order);
            }
        }
        let mut used = vec![false; n];
        order.reverse();
        for v in order {
            if !used[v] {
                let mut comps = vec![];
                dfs2(v, &mut used, g_rev, &mut comps);
                res.push(comps);
            }
        }

        let mut scc_id = vec![0; n];
        for i in 0..res.len() {
            for &j in &res[i] {
                scc_id[j] = i;
            }
        }
        SccResult {
            num_sccs: res.len(),
            scc_id,
            scc_comps: res,
        }
    }

    /// See [Radecki Algorithm!](https://codeforces.com/blog/entry/91608).
    ///
    /// **Note:** Does not support self-loops (edges of the form {u, u}).
    ///
    /// - `edge_updates`: Directed edges {u, v} where u -> v.
    /// - `n`: Number of vertices.
    ///
    /// Returns a vector `joins` where `joins[i]` is the minimum prefix of edges [0, joins[i]]
    /// for `eds[i][0]` and `eds[i][1]` to be in the same SCC. If `eds[i][0]` and `eds[i][1]` are never
    /// in the same SCC, then `joins[i] = m`.
    ///
    /// **Time Complexity:** O((n + m) log m).
    ///
    /// **Space Complexity:** O(n + m).
    ///
    /// # Examples
    ///
    /// ```
    /// let edge_updates = vec![(0, 1), (1, 2), (2, 0)];
    /// let n = 3;
    /// let joins = radecki_algorithm(&edge_updates, n);
    /// println!("{:?}", joins);
    /// ```
    ///
    /// Here, `joins` will contain the results as described.
    ///
    /// # Panics
    ///
    /// This function panics if `edge_updates` contains self-loops.
    pub fn offline_incremental_scc(edges: &Vec<(usize, usize)>, n: usize) -> Vec<usize> {
        let m = edges.len();
        let mut ids = vec![-1; n + m];
        let mut joins = vec![m; m];
        let eds = edges
            .iter()
            .enumerate()
            .map(|(i, x)| [x.0, x.1, i])
            .collect::<Vec<_>>();

        fn inner(
            mut current: Vec<[usize; 3]>,
            tl: usize,
            tr: usize,
            joins: &mut [usize],
            ids: &mut [i32],
        ) {
            let mid = (tl + tr) >> 1;
            let mut vs = vec![];
            let mut adj = vec![vec![]];
            let mut rev_adj = vec![vec![]];

            for [u, v, t] in current.iter_mut() {
                for w in [u.clone(), v.clone()] {
                    if ids[w] != -1 {
                        continue;
                    }
                    ids[w] = vs.len() as i32;
                    vs.push(w);
                    adj.push(vec![]);
                    rev_adj.push(vec![]);
                }
                *u = ids[*u] as usize;
                *v = ids[*v] as usize;
                if *t <= mid {
                    adj[*u].push(*v);
                    rev_adj[*v].push(*u);
                }
            }
            for v in vs {
                ids[v] = -1;
            }
            let scc_id = scc(&adj, &rev_adj).scc_id;
            let (left, mut right): (Vec<_>, Vec<_>) = current
                .into_iter()
                .partition(|x| scc_id[x[0]] == scc_id[x[1]]);
            for &t in &left {
                joins[t[2]] = joins[t[2]].min(mid);
            }
            if tr - tl == 1 {
                return;
            }
            for [u, v, t] in right.iter_mut() {
                *u = scc_id[*u as usize] as usize;
                *v = scc_id[*v as usize] as usize;
            }
            drop(adj);
            drop(rev_adj);
            drop(scc_id);

            inner(left, tl, mid, joins, ids);
            inner(right, mid, tr, joins, ids);
        }
        inner(eds.clone(), 0, m, &mut joins, &mut ids);
        joins
    }
}
