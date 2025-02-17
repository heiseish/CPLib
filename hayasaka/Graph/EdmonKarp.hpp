#include <bits/stdc++.h>
using namespace std;

/**
Max flow
*/
class EdmonKarp {
private:
	const int maxn = 1e5 + 3;
	int res[maxn][maxn];
	long long mf, f, s, t;
	vector<vector<pair<int, int> > > g;
	vector<int> p;

	void augment(int v, int minEdge) {
		if (v == s) {
			f = minEdge;
			return;
		} else if (p[v] != - 1) {
			augment(p[v], min(minEdge, res[p[v]][v]));
			res[p[v]][v] -= f;
			res[v][p[v]] += f;
		}
	}
public:
	long long solve() {
		mf = 0LL;
		while(1) {
			f = 0;
			bitset<maxn> vis; vis[s] = true;
			queue<int> q; q.push(s);
			p.assign(maxn, -1);
			while(!q.empty()) {
				int u = q.front(); q.pop();
				if (u == t) break;
				for (auto &v : g[u]) 
					if (res[u][v.st] > 0 && !vis[v.st]) 
						vis[v.st] = true, q.push(v.st), p[v.st] = u;
			}
			augment(t, inf);
			if (f == 0) break;
			mf += f;
		}
		return mf;
	}
}


