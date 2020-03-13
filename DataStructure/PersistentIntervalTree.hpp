#include <vector>
#include <algorithm>

class PersistentST {
private:
	struct node {
		int val, left, right;
	};
	int peak, ver;
	std::vector<int> root_ver, a;
	std::vector<node> it;
	int n;
	void buildTree(int l, int r) {
		if (l > r) return;
		if (l == r) {
			it[peak].val = a[l];
			return;
		}
		int mid = (l + r) >> 1, node= peak;
		it[node].left=++peak;
		buildTree(l, mid);
		it[node].right=++peak;
		buildTree(mid+1, r);
		it[node].val = max(it[it[node].left].val, it[it[node].right].val);
	}
	void update(int k, int l, int r, int pos, int x) {
		if(l > r || l > pos || r < pos) return;
		if(l==r) {
			it[peak].val=x;
			return;
		}
		int mid=(l+r)>>1, node=peak;
		if(pos > mid) {
			it[node].left=it[k].left;
			it[node].right=++peak;
			update(it[k].right, mid+1, r, pos, x);
		} else {
			it[node].left=++peak;
			it[node].right=it[k].right;
			update(it[k].left, l, mid, pos, x);
		}
		it[node].val = max(it[it[node].left].val, it[it[node].right].val);
	}

	int get(int k, int tl, int tr, int l, int r) {
		if (tl > tr || tl > r || tr < l) return 0;
		if (l <= tl && tr <= r) return it[k].val;
		int mid = (tl + tr) >> 1;
		return max(get(it[k].left, tl, mid, l, r), get(it[k].right, mid+1, tr, l, r));
	}
public:
	PersistentST(int _n) : n(_n) {
		a.resize(n);
		it.resize(20 * n); // log(n) * n
		root_ver.resize(n);
		peak=root_ver[0]=1;
		ver=0;
	};
	PersistentST(std::vector<int>& _a) : n(_a.size()), a(_a) {
		it.resize(20 * n);
		root_ver.resize(n);
		peak=root_ver[0]=1;
		ver=0;
		buildTree(1, n);
	}
	void update(int pos, int x) {
		root_ver[++ver] = ++peak;
		update(root_ver[ver-1], 1, n, pos, x);
	}
	int get(int l, int r, int kver=-1) {
		if (kver != -1) return get(root_ver[kver], 1, n, l, r);
		else return get(root_ver[ver], 1, n, l, r);
	}
	
};


// k-th smallest element in a range
class PersistentIT {
private:
	struct node {
		int val, left, right;
	};
	int peak, ver;
	std::vector<int> root_ver, a;
	std::vector<node> it;
	int n;
	void buildTree(int l, int r) {
		if (l > r) return;
		if (l == r) {
			it[peak].val = 0;
			return;
		}
		int mid = (l + r) >> 1, node= peak;
		it[node].left=++peak;
		buildTree(l, mid);
		it[node].right=++peak;
		buildTree(mid+1, r);
		it[node].val = it[it[node].left].val +
			it[it[node].right].val;
	}
	void update(int k, int l, int r, int pos, int x) {
		if(l > r || l > pos || r < pos) return;
		if(l==r) {
			it[peak].val += x;
			return;
		}
		int mid=(l+r)>>1, node=peak;
		if(pos > mid) {
			it[node].left=it[k].left;
			it[node].right=++peak;
			update(it[k].right, mid+1, r, pos, x);
		} else {
			it[node].left=++peak;
			it[node].right=it[k].right;
			update(it[k].left, l, mid, pos, x);
		}
		it[node].val = it[it[node].left].val +
			it[it[node].right].val;
	}

	int get(int kl, int kr, int tl, int tr, int kth) {
		if (tl > tr) return 0;
		if (tl == tr) {
			return tl;
		}
		int tm = (tl + tr) >> 1;
		int cur = it[it[kr].left].val - it[it[kl].left].val;
		if (cur >= kth)
			return get(it[kl].left, it[kr].left, tl, tm, kth);
		else 
			return get(it[kl].right, it[kr].right, tm+1, tr, kth - cur);
	}
public:
	PersistentIT(int _n) : n(_n) {
		a.assign(n, 0);
		it.resize(20 * n); // log(n) * n
		root_ver.resize(n);
		peak=root_ver[0]=1;
		ver=0;
		buildTree(1, n);
	};

	void update(int pos, int x) {
		root_ver[++ver] = ++peak;
		update(root_ver[ver-1], 1, n, pos, x);
	}
	int get(int l, int r, int k) {
		return get(root_ver[l-1], root_ver[r], 1, n, k);
	}
	
};