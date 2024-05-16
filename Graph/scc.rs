// 1-based
fn scc(n: usize, g: &Graph, g_rev: &Graph) -> Graph {
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
    let mut used = vec![false; n + 1];
    let mut order = vec![];

    let mut res = vec![];

    for i in 1..=n {
        if !used[i] {
            dfs1(i, &mut used, g, &mut order);
        }
    }

    let mut used = vec![false; n + 1];
    order.reverse();

    for v in order {
        if !used[v] {
            let mut comps = vec![];
            dfs2(v, &mut used, g_rev, &mut comps);
            res.push(comps);
        }
    }
    res
}
