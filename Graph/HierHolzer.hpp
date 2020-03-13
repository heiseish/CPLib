#include <bits/stdc++.h>
using namespace std;

/**
Finding Euler path for directed graph in O(E)
*/
class HierHolzer {
	static const int maxn = 1e5 + 3;
	vector<int> g[maxn];
public:
	void add_edge(int u, int v) {
		g[u].push_back(v);
	}
	
	vector<int> solve(int start) { 
		map<int, int> e; 
		for(int i = 0; i < n; ++i) e[i] = g[i].size();
		stack<int> ans; 
		vector<int> c; 
		ans.push(start); 
		int cur = start;
		while (!ans.empty()) { 
			if (e[cur]) { 
				ans.push(cur); 
				int next_v = g[cur].back(); 
				e[cur]--; 
				g[cur].pop_back(); 
				cur = next_v; 
			} else{ 
				c.push_back(cur); 
				cur = ans.top();
				ans.pop(); 
			} 
		} 

		if (c.size() != m + 1) return {}
		else {
			reverse(c.begin(), c.end());
			return c;
		}
	} 
}
