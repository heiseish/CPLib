mod dinic {
    // O(V ^ 2 * E)
    use std::collections::VecDeque;

    type LT = i64;
    type Graph = Vec<Vec<usize>>;

    #[derive(Default, Clone, Debug)]
    pub struct DinicFlowEdge {
        pub u: usize,
        pub v: usize,
        pub cap: LT,
        pub flow: LT,
    }

    impl DinicFlowEdge {
        pub fn new(u: usize, v: usize, cap: LT) -> Self {
            Self {
                v,
                u,
                cap,
                ..Default::default()
            }
        }
    }

    const flow_inf: LT = LT::MAX;

    #[derive(Default)]
    pub struct Dinic {
        n: usize,
        m: usize,
        adj: Graph,
        pub edges: Vec<DinicFlowEdge>,
    }

    fn bfs(
        t: usize,
        q: &mut VecDeque<usize>,
        adj: &Graph,
        edges: &[DinicFlowEdge],
        level: &mut [i32],
    ) -> bool {
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for &id in &adj[u] {
                if edges[id].cap - edges[id].flow < 1 {
                    continue;
                }
                if level[edges[id].v] != -1 {
                    continue;
                }
                level[edges[id].v] = level[u] + 1;
                q.push_back(edges[id].v);
            }
        }
        level[t] != -1
    }

    fn dfs(
        u: usize,
        t: usize,
        pushed: LT,
        level: &[i32],
        ptr: &mut [usize],
        edges: &mut [DinicFlowEdge],
        adj: &Graph,
    ) -> LT {
        if pushed == 0 {
            return 0;
        }
        if u == t {
            return pushed;
        }
        for cid in ptr[u]..adj[u].len() {
            let id = adj[u][cid];
            let v = edges[id].v;
            if level[u] + 1 != level[v] || edges[id].cap - edges[id].flow < 1 {
                continue;
            }
            let tr = dfs(
                v,
                t,
                pushed.min(edges[id].cap - edges[id].flow),
                level,
                ptr,
                edges,
                adj,
            );
            if tr == 0 {
                continue;
            }
            edges[id].flow += tr;
            edges[id ^ 1].flow -= tr;
            return tr;
        }
        0
    }

    impl Dinic {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                adj: vec![vec![]; n],
                ..Default::default()
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize, c: LT) {
            self.edges.push(DinicFlowEdge::new(u, v, c));
            self.edges.push(DinicFlowEdge::new(v, u, 0));
            self.adj[u].push(self.m);
            self.adj[v].push(self.m + 1);
            self.m += 2;
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> LT {
            let mut f = 0;
            let n = self.n;
            let mut q = VecDeque::new();

            loop {
                let mut level = vec![-1; n];
                level[s] = 0;
                q.push_back(s);
                if !bfs(t, &mut q, &self.adj, &self.edges, &mut level) {
                    break;
                }

                let mut ptr = vec![0; n];
                let mut pushed = dfs(s, t, flow_inf, &level, &mut ptr, &mut self.edges, &self.adj);
                while pushed > 0 {
                    f += pushed;
                    pushed = dfs(s, t, flow_inf, &level, &mut ptr, &mut self.edges, &self.adj);
                }
            }
            f
        }
    }
}

mod push_relabel {
    // Despite the simplicity, this modification reduces the complexity by a lot. To be precise, the complexity of the resulting algorithm is
    // $O(V E + V^2 \sqrt{E})$ , which in the worst case is
    // $O(V^3)$ .
    type LT = i64;
    type TwoDVec<T> = Vec<Vec<T>>;

    // 0-based
    pub struct PushRelabel {
        n: usize,
        capacity: TwoDVec<LT>,
    }

    fn push(u: usize, v: usize, excess: &mut [LT], flow: &mut TwoDVec<LT>, capacity: &TwoDVec<LT>) {
        let d = excess[u].min(capacity[u][v] - flow[u][v]);
        flow[u][v] += d;
        flow[v][u] -= d;
        excess[u] -= d;
        excess[v] += d;
    }

    fn relabel(u: usize, n: usize, height: &mut [LT], flow: &TwoDVec<LT>, capacity: &TwoDVec<LT>) {
        let mut d = LT::MAX;
        for i in 0..n {
            if capacity[u][i] - flow[u][i] > 0 {
                d = d.min(height[i]);
            }
        }
        if d != LT::MAX {
            height[u] = d + 1;
        }
    }

    fn find_max_height_vertices(
        s: usize,
        t: usize,
        n: usize,
        excess: &[LT],
        height: &[LT],
    ) -> Vec<usize> {
        let mut max_height = vec![];
        for i in 0..n {
            if i != s && i != t && excess[i] > 0 {
                if !max_height.is_empty() && height[i] > height[max_height[0]] {
                    max_height.clear();
                }
                if max_height.is_empty() || height[i] == height[max_height[0]] {
                    max_height.push(i);
                }
            }
        }
        max_height
    }

    pub struct PRMaxFlowResult {
        pub max_flow: LT,
        pub flow: TwoDVec<LT>,
        pub excess: Vec<LT>,
    }
    impl PushRelabel {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                capacity: vec![vec![0; n]; n],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize, c: LT) {
            self.capacity[u][v] += c;
        }

        pub fn max_flow(&self, s: usize, t: usize) -> PRMaxFlowResult {
            let n = self.n;
            let mut height = vec![0; n];
            height[s] = n as LT;
            let mut flow = vec![vec![0; n]; n];
            let mut excess = vec![0; n];
            excess[s] = LT::MAX;
            for i in 0..n {
                if i != s {
                    push(s, i, &mut excess, &mut flow, &self.capacity);
                }
            }
            let mut current = find_max_height_vertices(s, t, n, &excess, &height);
            while !current.is_empty() {
                for i in current {
                    let mut pushed = false;
                    for j in 0..n {
                        if excess[i] == 0 {
                            break;
                        }
                        if self.capacity[i][j] - flow[i][j] > 0 && height[i] == height[j] + 1 {
                            push(i, j, &mut excess, &mut flow, &self.capacity);
                            pushed = true;
                        }
                    }
                    if !pushed {
                        relabel(i, n, &mut height, &flow, &self.capacity);
                        break;
                    }
                }
                current = find_max_height_vertices(s, t, n, &excess, &height);
            }
            PRMaxFlowResult {
                max_flow: excess[t],
                excess,
                flow,
            }
        }
    }
}


mod mpm {
    // INCOMPLETE
    // O(V ^ 3)
    use std::collections::VecDeque;

    type LT = i64;
    type Graph = Vec<Vec<usize>>;

    #[derive(Default, Clone, Debug)]
    pub struct FlowEdge {
        pub u: usize,
        pub v: usize,
        pub cap: LT,
        pub flow: LT,
    }

    impl FlowEdge {
        pub fn new(u: usize, v: usize, cap: LT) -> Self {
            Self {
                v,
                u,
                cap,
                ..Default::default()
            }
        }
    }

    const flow_inf: LT = LT::MAX;

    #[derive(Default)]
    pub struct MPM {
        n: usize,
        m: usize,
        adj: Graph,
        pub edges: Vec<FlowEdge>,
    }

    fn bfs(
        qh: &mut usize, 
        qt: &mut usize, 
        q: &mut[usize], 
        adj: &Graph, 
        edges: &[FlowEdge], 
        level: &mut[i32]) -> bool
    {
        while qh < qt
        {
            let u = q[qh];
            gh += 1;
            for &id in &adj[u] {
                if edges[id].cap - edges[id].flow < 1 {
                    continue;
                }
                if level[edges[id].v] != -1 {
                    continue;
                }
                level[edges[id].v] = level[u] + 1;
                q[qt] = edges[id].v;
                qt += 1;
            }
        }
        level[t] != -1
    }

    fn remove_node(v: usize, 
            lin: &mut [LinkedList<usize>], 
            lout: &mut [LinkedList<usize>],
            pin: &mut[PT],
            pout: &mut[PT]
        ) {
        for &i : &lin[v] {
            let v = edges[i].u;
            let idx = lout[v].iter().position(|x| *x == i).unwrap();
            lout[v].remove(idx);
            pout[u] -= edges[i].cap - edges[i].flow;
        }

        for &i : &lout[u] {
            let v = edges[i].v;
            let idx = lin[v].iter().position(|x| *x == i).unwrap();
            lin[v].remove(idx);
            pin[u] -= edges[i].cap - edges[i].flow;
        }
    }

    fn pot(v: usize, pin: & [usize], pout: & [usize]) -> LT
    {
        pin[v].min(pout[v]);
    }

    fn push(from: usize, to: usize, f: LT, forw: bool, n: usize,q: &mut[usize]) 
    {
        let qh = 0;
        let qt = 0;
        let mut ex = vec![0; n];
        ex[from] = f;
        q[qt] = from;
        qt += 1;
        while qh < qt
        {
            let v = q[qh];
            qh += 1;
            if v == to {
                break;
            }
           
            let must = ex[v];
            auto it = forw ? out[v].begin() : in[v].begin();
            loop
            {
                int u = forw ? edges[*it].u : edges[*it].v;
                long long pushed = min(must, edges[*it].cap - edges[*it].flow);
                if (pushed == 0)
                    break;
                if (forw)
                {
                    pout[v] -= pushed;
                    pin[u] -= pushed;
                }
                else
                {
                    pin[v] -= pushed;
                    pout[u] -= pushed;
                }
                if (ex[u] == 0)
                    q[qt++] = u;
                ex[u] += pushed;
                edges[*it].flow += pushed;
                edges[(*it) ^ 1].flow -= pushed;
                must -= pushed;
                if (edges[*it].cap - edges[*it].flow == 0)
                {
                    auto jt = it;
                    ++jt;
                    if (forw)
                    {
                        in[u].erase(find(in[u].begin(), in[u].end(), *it));
                        out[v].erase(it);
                    }
                    else
                    {
                        out[u].erase(find(out[u].begin(), out[u].end(), *it));
                        in[v].erase(it);
                    }
                    it = jt;
                }
                else
                    break;
                if (!must)
                    break;
            }
        }
    }


    impl MPM {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                adj: vec![vec![]; n],
                ..Default::default()
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize, c: LT) {
            self.edges.push(DinicFlowEdge::new(u, v, c));
            self.edges.push(DinicFlowEdge::new(v, u, 0));
            self.adj[u].push(self.m);
            self.adj[v].push(self.m + 1);
            self.m += 2;
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> LT {
            let mut ans = 0;
            let n = self.n;
            let m = self.m;
            let mut q = vec![0; n];
            let mut lin = vec![LinkedList::new();n];
            let mut lout = vec![LinkedList::new();n];
            while (true)
            {
                let mut pin = vec![0; n];
                let mut pout = vec![0; n];
                let mut level = vec![-1; n];
                let mut alive = vec![true; n];
                level[s] = 0;
                let mut qh = 0;
                let mut qt = 1;
                q[0] = s;

                if !bfs(&mut qh, &mut qt, &mut q, &self.adj, &self.edges,
                    &mut level) {
                    break;
                }
                
                for i in 0..n {
                    lin[i].clear();
                    lout[i].clear();
                }
                for i in 0..m {
                    if edges[i].cap - edges[i].flow == 0 {
                        continue;
                    }
                    let u = edges[i].u;
                    let v = edges[i].v;

                    if level[u] + 1 == level[v] && (level[v] < level[t] || v == t) {
                        lin[v].push_back(i);
                        lout[u].push_back(i);
                        pin[v] += edges[i].cap - edges[i].flow;
                        pout[u] += edges[i].cap - edges[i].flow;
                    }
                }
                pout[t] = flow_inf;
                pin[s]  = flow_inf;

                loop {
                    let mut v = -1;
                    for i in 0..n {
                        if !alive[i] {
                            continue;
                        }
                     
                        if v == -1 || pot(i, &pin, &pout) < pot(v as usize, &pin, &pout) {
                            v = i;
                        }
                 
                    }
                    if v == -1 {
                        break;
                    }
                    if pot(v as usize, &pin, &pout) == 0
                    {
                        alive[v as usize] = false;
                        remove_node(v, &mut lin, &mut lout, &mut pin, &mut pout);
                        continue;
                    }
                    let f = pot(v as usize, &pin, &pout);
                    ans += f;
                    push(v, s, f, false);
                    push(v, t, f, true);
                    alive[v] = false;
                    remove_node(v, &mut lin, &mut lout, &mut pin, &mut pout);
                }
            }
            return ans;
        }
    }

}