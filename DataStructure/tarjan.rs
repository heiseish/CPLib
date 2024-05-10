mod tarjan {
    use crate::Graph;
    use crate::VecDeque;

    #[inline(always)]
    fn dfs1(v: usize, g: &Graph, used: &mut Vec<bool>, order: &mut VecDeque<usize>) {
        used[v] = true;
        for u in g.get_neighbors(v) {
            if !used[*u] {
                dfs1(*u, g, used, order);
            }
        }
        order.push_front(v);
    }

    #[inline(always)]
    fn dfs2(v: usize, rev_g: &Graph, used: &mut Vec<bool>, components: &mut Vec<usize>) {
        used[v] = true;
        components.push(v);

        for u in rev_g.get_neighbors(v) {
            if !used[*u] {
                dfs2(*u, rev_g, used, components);
            }
        }
    }

    pub fn run(g: &Graph, comps: &mut Vec<usize>) -> usize {
        let rev = Graph::rev(&g);
        let mut used = vec![false; g.len()];
        let mut order = VecDeque::with_capacity(g.len());
        for i in 0..g.len() {
            if !used[i] {
                dfs1(i, &g, &mut used, &mut order);
            }
        }
        let mut used = vec![false; g.len()];
        let mut components = Vec::with_capacity(g.len());
        let mut cnt = 0;
        for v in order {
            if !used[v] {
                dfs2(v, &rev, &mut used, &mut components);
                for c in &components {
                    comps[*c] = cnt;
                }
                cnt += 1;
                components.clear();
            }
        }
        cnt
    }
}
