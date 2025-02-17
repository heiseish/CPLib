#include <vector>

using namespace std;
/**
 * Hungarian algorithm
The assignment problem has two equivalent statements:

Given a square matrix A[1..N,1..N], you need to select N elements in it so that exactly one element is selected in 
each row and column, and the sum of the values of these elements is the smallest.
There are N orders and N machines. The cost of manufacturing on each machine is known for each order. 
Only one order can be performed on each machine. It is required to assign all orders to the machines so that the total cost is minimized.
*/
vector<long> assignment(vector<vector<long>>& a) {
	const static long INF = 1e11;
    long n = a.size();
    long m = n * 2 + 2;
    vector<vector<long>> f(m, vector<long>(m));
    long s = m - 2, t = m - 1;
    long cost = 0;
    while (true) {
        vector<long> dist(m, INF);
        vector<long> p(m);
        vector<bool> inq(m, false);
        queue<long> q;
        dist[s] = 0;
        p[s] = -1;
        q.emplace(s);
        while (!q.empty()) {
            long v = q.front();
            q.pop();
            inq[v] = false;
            if (v == s) {
                for (long i = 0; i < n; ++i) {
                    if (f[s][i] == 0) {
                        dist[i] = 0;
                        p[i] = s;
                        inq[i] = true;
                        q.push(i);
                    }
                }
            } else {
                if (v < n) {
                    for (long j = n; j < n + n; ++j) {
                        if (f[v][j] < 1 && dist[j] > dist[v] + a[v][j - n]) {
                            dist[j] = dist[v] + a[v][j - n];
                            p[j] = v;
                            if (!inq[j]) {
                                q.push(j);
                                inq[j] = true;
                            }
                        }
                    }
                } else {
                    for (long j = 0; j < n; ++j) {
                        if (f[v][j] < 0 && dist[j] > dist[v] - a[j][v - n]) {
                            dist[j] = dist[v] - a[j][v - n];
                            p[j] = v;
                            if (!inq[j]) {
                                q.push(j);
                                inq[j] = true;
                            }
                        }
                    }
                }
            }
        }

        long curcost = INF;
        for (long i = n; i < n + n; ++i) {
            if (f[i][t] == 0 && dist[i] < curcost) {
                curcost = dist[i];
                p[t] = i;
            }
        }
        if (curcost == INF)
            break;
        cost += curcost;
        for (long cur = t; cur != -1; cur = p[cur]) {
            long prev = p[cur];
            if (prev != -1)
                f[cur][prev] = -(f[prev][cur] = 1);
        }
    }

    vector<long> answer(n);
    for (long i = 0; i < n; ++i) {
        for (long j = 0; j < n; ++j) {
            if (f[i][j + n] == 1)
                answer[i] = j;
        }
    }
    return answer;
}


class MinCostMaxFlow {
private:
	struct Edge {
		int from, to, capacity, cost;
	};
	int N;
	vector<Edge> edges;
	std::vector<std::vector<int> > adj, cost, capacity;
	const int INF = 1e9;
	void shortest_paths(int n, int v0, std::vector<int>& d, std::vector<int>& p) {
		d.assign(n, INF);
		d[v0] = 0;
		vector<int> m(n, 2);
		deque<int> q;
		q.push_back(v0);
		p.assign(n, -1);

		while (!q.empty()) {
			int u = q.front();
			q.pop_front();
			m[u] = 0;
			for (int v : adj[u]) {
				if (capacity[u][v] > 0 && d[v] > d[u] + cost[u][v]) {
					d[v] = d[u] + cost[u][v];
					p[v] = u;
					if (m[v] == 2) {
						m[v] = 1;
						q.push_back(v);
					} else if (m[v] == 0) {
						m[v] = 1;
						q.push_front(v);
					}
				}
			}
		}
	}

public:
	MinCostMaxFlow(int _N) : N(_N) {}
	void add_edge(int from, int to, int capacity, int cost) {
		edges.push_back({ from, to, capacity, cost });
	}
	int min_cost_flow(int max_flow, int s, int t) {
		adj.assign(N, vector<int>());
		cost.assign(N, vector<int>(N, 0));
		capacity.assign(N, vector<int>(N, 0));
		for (Edge& e : edges) {
			adj[e.from].push_back(e.to);
			adj[e.to].push_back(e.from);
			cost[e.from][e.to] = e.cost;
			cost[e.to][e.from] = -e.cost;
			capacity[e.from][e.to] = e.capacity;
		}

		int flow = 0;
		int cost = 0;
		std::vector<int> d, p;
		while (flow < max_flow) {
			shortest_paths(N, s, d, p);
			if (d[t] == INF)
				break;

			// find max flow on that path
			int f = max_flow - flow;
			int cur = t;
			while (cur != s) {
				f = min(f, capacity[p[cur]][cur]);
				cur = p[cur];
			}

			// apply flow
			flow += f;
			cost += f * d[t];
			cur = t;
			while (cur != s) {
				capacity[p[cur]][cur] -= f;
				capacity[cur][p[cur]] += f;
				cur = p[cur];
			}
		}

		if (flow < max_flow)
			return -1;
		else
			return cost;
	}
};