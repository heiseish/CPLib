#include <bits/stdc++.h>
#include "SegmentTree.hpp"
using namespace std;

int main() {
	ios_base::sync_with_stdio(false); cin.tie(0);
	#ifdef LOCAL_PROJECT
		freopen("input.txt","r",stdin);
		int tt = clock();
	#endif
	
	int n = 10;
	vector<int> a(n+1), b;
	for(int i = 1; i <= n;++i) a[i] = rand() % 1000;

	for (int i = 1; i <= n;++i) {
		b.push_back(a[i]);
	}
	sort(b.begin(), b.end());
	map<int, int> ma, re;
	int cur = 1;
	for (auto &v : b) {
		ma[v] = cur;
		re[cur++] = v;
	}
	for(int i = 1; i <= n; ++i) cout << a[i] << ' ';
	cout <<'\n';
	for(int i = 1; i <= n; ++i) cout << ma[a[i]] << ' ';
	cout <<'\n';

	PersistentIT stm(n+1);
	for(int i = 1; i <= n; ++i) stm.update(ma[a[i]], 1);

	cout <<  stm.get(1, 1, 1) <<'\n';
	cout <<  stm.get(1, 5, 4) <<'\n';
	cout <<  stm.get(1, 8, 7) <<'\n';
	cout << stm.get(1, 8, 1) <<'\n';

	#ifdef LOCAL_PROJECT
		cerr << "TIME = " << (double) 1.0 * (clock() - tt) / CLOCKS_PER_SEC << " seconds\n";
		tt = clock();
	#endif
}