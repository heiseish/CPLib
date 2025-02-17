type Graph = Vec<Vec<usize>>;

mod bridges {
    fn find_bridges(n: usize, g: &Graph) {
        fn dfs(
            u: usize,
            p: i32,
            g: &Graph,
            grps: &mut [usize],
            grp: usize,
            vis: &mut [bool],
            tin: &mut [usize],
            low: &mut [usize],
            timer: &mut usize,
            bl: &mut [Vec<usize>],
            szs: &mut [usize],
        ) -> usize {
            let mut sz = 1;
            vis[u] = true;
            tin[u] = *timer;
            low[u] = *timer;
            *timer += 1;
            grps[u] = grp;

            for &v in &g[u] {
                if v as i32 == p {
                    continue;
                }
                if vis[v] {
                    low[u] = low[u].min(tin[v]);
                } else {
                    sz += dfs(v, u as i32, g, grps, grp, vis, tin, low, timer, bl, szs);
                    low[u] = low[u].min(low[v]);
                    if low[v] > tin[u] {
                        bl[grps[u]].push(v);
                    }
                }
            }
            szs[u] = sz;
            sz
        }

        let mut grps = vec![0; n + 3];
        let mut grps_size = vec![0; n + 3];
        let mut szs = vec![0; n + 3];
        let mut tin = vec![0; n + 3];
        let mut vis = vec![false; n + 3];
        let mut low = vec![0; n + 3];
        let mut ble = vec![vec![]; n + 3];
        let mut timer = 0;

        let mut grp = 1;

        for u in 1..=n {
            if !vis[u] {
                dfs(
                    u, -1, &g, &mut grps, grp, &mut vis, &mut tin, &mut low, &mut timer, &mut ble,
                    &mut szs,
                );
                grp += 1;
            }
        }
        // group and grp size are kinda like connected component/DSU
        for u in 1..=n {
            grps_size[grps[u]] += 1;
        }
    }
}
