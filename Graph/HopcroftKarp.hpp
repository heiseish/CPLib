#include <vector>
#include <queue>
#include <pair>

class HopcroftKarp {
private:
	static const NIL = 0;
	std::vector<std::vector<int> > G;
	int n, m;
	std::vector<int> match, dist;
	// n: number of nodes on left side, nodes are numbered 1 to n
	// m: number of nodes on right side, nodes are numbered n+1 to n+m
	// G = NIL[0] ∪ G1[G[1---n]] ∪ G2[G[n+1---n+m]]
	bool bfs() {
		int i, u, v, len;
		queue<int> Q;
		for(i=1; i<=n; i++) {
			if(match[i]==NIL) {
				dist[i] = 0;
				Q.push(i);
			}
			else dist[i] = INF;
		}
		dist[NIL] = INF;
		while(!Q.empty()) {
			u = Q.front(); Q.pop();
			if(u!=NIL) {
				len = G[u].size();
				for(i=0; i<len; i++) {
					v = G[u][i];
					if(dist[match[v]]==INF) {
						dist[match[v]] = dist[u] + 1;
						Q.push(match[v]);
					}
				}
			}
		}
		return (dist[NIL]!=INF);
	}

	bool dfs(int u) {
		int i, v, len;
		if(u!=NIL) {
			len = G[u].size();
			for(i=0; i<len; i++) {
				v = G[u][i];
				if(dist[match[v]]==dist[u]+1) {
					if(dfs(match[v])) {
						match[v] = u;
						match[u] = v;
						return true;
					}
				}
			}
			dist[u] = INF;
			return false;
		}
		return true;
	}
public:
	HopcroftKarp(int _n) : n(_n) {
		G.resize(n + 1);
		dist.resize(n + 1);
		match.assign(n + 1, NIL);
	}

	int get_matching() {
		int matching = 0, i;
		// match[] is assumed NIL for all vertex in G
		while(bfs())
			for(i=1; i<=n; i++)
				if(match[i]==NIL && dfs(i))
					matching++;
		return matching;
	}

	/**
	Recover the matching pair from hopcroft craft
	for https://open.kattis.com/problems/bilateral
	*/
	std::vector<std::pair<int, int> > getMatching() {
		std::vector<int> ok(n+1, 0);
		while(true) {
			bool can = false;
			for(int i = 1; i <= n; ++i) {
				for (auto &v : G[i]) {
					if (match[i] == NIL && match[v] != NIL && ok[v] == 0) {
						ok[v]=1;
						match[match[v]] = NIL;
						can = true;
					}
				}
			}
			if (!can) break;
		}
		for(int i = 1; i <= n; ++i)
			if (match[i] != NIL && !ok[i] && !ok[match[i]]) 
				ok[i] = 1;
		std::vector<std::pair<int, int> > ans;
		for(int i = 1; i <= n; ++i) 
			if (ok[i]) 
				ans.push_back({i, match[i] });
		return ans;
	}
}
