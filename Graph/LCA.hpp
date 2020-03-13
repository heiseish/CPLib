#include <vector>
#include <utility>

/**
* The Farach-Colton and Bender algorithm
* Based on optimized sparse table
* O(N) preprocessing and O(1) query
*/
class LCA {
private:
	int n;
	std::vector<std::vector<int>> adj;

	int block_size, block_cnt;
	std::vector<int> first_visit;
	std::vector<int> euler_tour;
	std::vector<int> height;
	std::vector<int> log_2;
	std::vector<std::vector<int>> st;
	std::vector<std::vector<std::vector<int>>> blocks;
	std::vector<int> block_mask;

public:
	LCA(int _n) : n (_n) {
		adj.resize(n + 1);
	}
	void addEdge(int a, int b) {
		adj[a].emplace_back(b);
	}
	void dfs(int v, int p, int h) {
	    first_visit[v] = euler_tour.size();
	    euler_tour.push_back(v);
	    height[v] = h;
	    for (int u : adj[v]) {
	        if (u == p)
	            continue;
	        dfs(u, v, h + 1);
	        euler_tour.push_back(v);
	    }
	}

	int min_by_h(int i, int j) {
	    return height[euler_tour[i]] < height[euler_tour[j]] ? i : j;
	}

	void precompute_lca(int root) {
	    // get euler tour & indices of first occurences
	    first_visit.assign(n, -1);
	    height.assign(n, 0);
	    euler_tour.reserve(2 * n);
	    dfs(root, -1, 0);

	    // precompute all log values
	    int m = euler_tour.size();
	    log_2.reserve(m + 1);
	    log_2.push_back(-1);
	    for (int i = 1; i <= m; i++)
	        log_2.push_back(log_2[i / 2] + 1);

	    block_size = max(1, log_2[m] / 2);
	    block_cnt = (m + block_size - 1) / block_size;

	    // precompute minimum of each block and build sparse table
	    st.assign(block_cnt, std::vector<int>(log_2[block_cnt] + 1));
	    for (int i = 0, j = 0, b = 0; i < m; i++, j++) {
	        if (j == block_size)
	            j = 0, b++;
	        if (j == 0 || min_by_h(i, st[b][0]) == i)
	            st[b][0] = i;
	    }
	    for (int l = 1; l <= log_2[block_cnt]; l++) {
	        for (int i = 0; i < block_cnt; i++) {
	            int ni = i + (1 << (l - 1));
	            if (ni >= block_cnt)
	                st[i][l] = st[i][l-1];
	            else
	                st[i][l] = min_by_h(st[i][l-1], st[ni][l-1]);
	        }
	    }

	    // precompute mask for each block
	    block_mask.assign(block_cnt, 0);
	    for (int i = 0, j = 0, b = 0; i < m; i++, j++) {
	        if (j == block_size)
	            j = 0, b++;
	        if (j > 0 && (i >= m || min_by_h(i - 1, i) == i - 1))
	            block_mask[b] += 1 << (j - 1);
	    }

	    // precompute RMQ for each unique block
	    int possibilities = 1 << (block_size - 1);
	    blocks.resize(possibilities);
	    for (int b = 0; b < block_cnt; b++) {
	        int mask = block_mask[b];
	        if (!blocks[mask].empty())
	            continue;
	        blocks[mask].assign(block_size, std::vector<int>(block_size));
	        for (int l = 0; l < block_size; l++) {
	            blocks[mask][l][l] = l;
	            for (int r = l + 1; r < block_size; r++) {
	                blocks[mask][l][r] = blocks[mask][l][r - 1];
	                if (b * block_size + r < m)
	                    blocks[mask][l][r] = min_by_h(b * block_size + blocks[mask][l][r], 
	                            b * block_size + r) - b * block_size;
	            }
	        }
	    }
	}

	int lca_in_block(int b, int l, int r) {
	    return blocks[block_mask[b]][l][r] + b * block_size;
	}

	int lca(int v, int u) {
	    int l = first_visit[v];
	    int r = first_visit[u];
	    if (l > r)
	        swap(l, r);
	    int bl = l / block_size;
	    int br = r / block_size;
	    if (bl == br)
	        return euler_tour[lca_in_block(bl, l % block_size, r % block_size)];
	    int ans1 = lca_in_block(bl, l % block_size, block_size - 1);
	    int ans2 = lca_in_block(br, 0, r % block_size);
	    int ans = min_by_h(ans1, ans2);
	    if (bl + 1 < br) {
	        int l = log_2[br - bl - 1];
	        int ans3 = st[bl+1][l];
	        int ans4 = st[br - (1 << l)][l];
	        ans = min_by_h(ans, min_by_h(ans3, ans4));
	    }
	    return euler_tour[ans];
	}
};

/**
* Tarjan DSU algorithm
*/
struct LCAOffLine {
	vector<vector<int>> adj;
	vector<vector<int>> queries;
	vector<int> ancestor;
	vector<bool> visited;

	void dfs(int v)
	{
	    visited[v] = true;
	    ancestor[v] = v;
	    for (int u : adj[v]) {
	        if (!visited[u]) {
	            dfs(u);
	            union_sets(v, u);
	            ancestor[find_set(v)] = v;
	        }
	    }
	    for (int other_node : queries[v]) {
	        if (visited[other_node])
	            cout << "LCA of " << v << " and " << other_node
	                 << " is " << ancestor[find_set(other_node)] << ".\n";
	    }
	}

	void compute_LCAs() {
	    // initialize n, adj and DSU
	    // for (each query (u, v)) {
	    //    queries[u].push_back(v);
	    //    queries[v].push_back(u);
	    // }

	    ancestor.resize(n);
	    visited.assign(n, false);
	    dfs(0);
	}
}
class LCASegmentTree {
private:
	std::vector<std::vector<int>> adj;
    std::vector<int> height, euler, first, segtree;
    std::vector<bool> visited;
    int n;
    void dfs(int node, int h = 0) {
        visited[node] = true;
        height[node] = h;
        first[node] = euler.size();
        euler.push_back(node);
        for (auto to : adj[node]) {
            if (!visited[to]) {
                dfs(adj, to, h + 1);
                euler.push_back(node);
            }
        }
    }
public:
    LCA(std::vector<std::vector<int>>& _adj, int root = 0) : adj(_adj), n(adj.size())  {
        height.resize(n);
        first.resize(n);
        euler.reserve(n * 2);
        visited.assign(n, false);
        dfs(root);
        int m = euler.size();
        segtree.resize(m * 4);
        build(1, 0, m - 1);
    }

   
    void build(int node, int b, int e) {
        if (b == e) {
            segtree[node] = euler[b];
        } else {
            int mid = (b + e) / 2;
            build(node << 1, b, mid);
            build(node << 1 | 1, mid + 1, e);
            int l = segtree[node << 1], r = segtree[node << 1 | 1];
            segtree[node] = (height[l] < height[r]) ? l : r;
        }
    }

    int query(int node, int b, int e, int L, int R) {
        if (b > R || e < L)
            return -1;
        if (b >= L && e <= R)
            return segtree[node];
        int mid = (b + e) >> 1;

        int left = query(node << 1, b, mid, L, R);
        int right = query(node << 1 | 1, mid + 1, e, L, R);
        if (left == -1) return right;
        if (right == -1) return left;
        return height[left] < height[right] ? left : right;
    }

    int lca(int u, int v) {
        int left = first[u], right = first[v];
        if (left > right)
            std::swap(left, right);
        return query(1, 0, euler.size() - 1, left, right);
    }
};


struct LCABinaryLifting {
	int n, l;
	std::vector<std::vector<int>> adj;

	int timer;
	std::vector<int> tin, tout;
	std::vector<std::vector<int>> up;

	void dfs(int v, int p)
	{
	    tin[v] = ++timer;
	    up[v][0] = p;
	    for (int i = 1; i <= l; ++i)
	        up[v][i] = up[up[v][i-1]][i-1];

	    for (int u : adj[v]) {
	        if (u != p)
	            dfs(u, v);
	    }

	    tout[v] = ++timer;
	}

	bool is_ancestor(int u, int v)
	{
	    return tin[u] <= tin[v] && tout[u] >= tout[v];
	}

	int lca(int u, int v)
	{
	    if (is_ancestor(u, v))
	        return u;
	    if (is_ancestor(v, u))
	        return v;
	    for (int i = l; i >= 0; --i) {
	        if (!is_ancestor(up[u][i], v))
	            u = up[u][i];
	    }
	    return up[u][0];
	}

	void preprocess(int root) {
	    tin.resize(n);
	    tout.resize(n);
	    timer = 0;
	    l = ceil(log2(n));
	    up.assign(n, std::vector<int>(l + 1));
	    dfs(root, root);
	}
}