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
