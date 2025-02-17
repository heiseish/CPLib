#include <vector>
#include <utility>

const int N = 1e6 + 7;
struct Node {
    int x, y;
    Node(int _x, int _y) : x(_x), y(_y){}
    Node() : x(0), y(0) {}
};
int n;
int st[N];
int a[N];
int lazy[N << 2];
Node t[N << 2];

Node combine(Node lhs, Node rhs) {
    Node temp;
    temp.x = min(lhs.x, rhs.x);
    temp.y = max(lhs.y, rhs.y);
    return temp;
}

auto build(int v, int l, int r) -> void {
    if (l == r) {
        t[v] = a[l];
        return;
    }
    int mid = (l + r) >> 1;
    build(v << 1, l , mid);
    build(v << 1 | 1, mid + 1, r);  
    t[v] = combine(t[v << 1], t[v << 1 | 1]); 
}
auto push(int v, int l, int r) -> void{
    t[v].x += lazy[v];
    t[v].y += lazy[v];
    if (l < r) {
        lazy[v << 1] += lazy[v];
        lazy[v << 1 | 1] += lazy[v];
    }
    lazy[v] = 0;
}
auto update(int v, int l, int r, int x, int y, int to_add) -> void {
    push(v, l, r);
    if (l > y || r < x) return;
    if (l >= x && r <= y) {
        lazy[v] += to_add;
        push(v, l, r);
        return;
    } 
    int mid = (l + r) >> 1;
    update(v << 1, l, mid, x, y, to_add);
    update(v << 1 |  1, mid+1, r, x, y, to_add);
    t[v] = combine(t[v << 1], t[v << 1 | 1]);
}
 
auto get(int v, int l, int r, int x, int y) -> Node {
    if (l > y || r < x) return {INF, -INF};
    push(v, l, r);
    if (l >= x && r <= y) return t[v];
    int mid = (l + r) >> 1;
    return combine(get(v << 1, l, mid, x, y), 
            get(v << 1 |  1, mid+1, r, x, y));
}
 

 

//Find the smallest number greater or equal to a specified number. No modification queries.
class MergeSortTree {
private:
	int n;
	std::vector<std::vector<int> >t;
	std::vector<int> a;
	const int INF = 1 << 30;
	void build(int v, int tl, int tr) {
		if (tl == tr) {
			t[v] = std::vector<int>(1, a[tl]);
		} else { 
			int tm = (tl + tr) / 2;
			build(v*2, tl, tm);
			build(v*2+1, tm+1, tr);
			merge(t[v*2].begin(), t[v*2].end(), t[v*2+1].begin(), t[v*2+1].end(),
				back_inserter(t[v]));
		}
	}

	int get(int v, int tl, int tr, int l, int r, int x) {
		if (l > r)
			return INF;
		if (l == tl && r == tr) {
			auto pos = lower_bound(t[v].begin(), t[v].end(), x);
			if (pos != t[v].end())
				return *pos;
			return INF;
		}
		int tm = (tl + tr) / 2;
		return min(get(v*2, tl, tm, l, min(r, tm), x), 
				get(v*2+1, tm+1, tr, max(l, tm+1), r, x));
	}
public:
	MergeSortTree(int _n) : n(_n) {
		t.resize(n * 4);
		a.assign(n, 0);
	}

	MergeSortTree(std::vector<int>& _a) : n(_a.size()), a(_a) {
		t.resize(n * 4);
		build(1, 1, n);
	}

	int get(int l, int r, int x) {
		return get(1, 1, n, l, r, x);
	}

};

// Find the smallest number greater or equal to a specified number. With modification queries.
// Can apply fractional cascading
class MergeSortTreeWithModification {
private:
	int n;
	std::vector<set<int> >t;
	std::vector<int> a;
	const int INF = 1 << 30;
	void build(int v, int tl, int tr) {
		if (tl == tr) {
			t[v].insert(a[tl]);
		} else { 
			int tm = (tl + tr) / 2;
			build(v*2, tl, tm);
			build(v*2+1, tm+1, tr);
			t[v].insert(t[v*2].begin(), t[v*2].end());
			t[v].insert(t[v*2+1].begin(), t[v*2+1].end());
		}
	}

	int get(int v, int tl, int tr, int l, int r, int x) {
		if (l > r)
			return INF;
		if (l == tl && r == tr) {
			auto pos = t[v].lower_bound(x);
			if (pos != t[v].end())
				return *pos;
			return INF;
		}
		int tm = (tl + tr) / 2;
		return min(get(v*2, tl, tm, l, min(r, tm), x), 
				get(v*2+1, tm+1, tr, max(l, tm+1), r, x));
	}

	void update(int v, int tl, int tr, int pos, int new_val) {
		t[v].erase(t[v].find(a[pos]));
		t[v].insert(new_val);
		if (tl != tr) {
			int tm = (tl + tr) / 2;
			if (pos <= tm)
				update(v*2, tl, tm, pos, new_val);
			else
				update(v*2+1, tm+1, tr, pos, new_val);
		} else {
			a[pos] = new_val;
		}
	}
public:
	MergeSortTreeWithModification(int _n) : n(_n) {
		t.resize(n * 4);
		a.assign(n, 0);
	}

	MergeSortTreeWithModification(std::vector<int>& _a) : n(_a.size()), a(_a) {
		t.resize(n * 4);
		build(1, 1, n);
	}

	int get(int l, int r, int x) {
		return get(1, 1, n, l, r, x);
	}

	void update(int pos, int new_val) {
		update(1, 1, n, pos, new_val);
	}

};


// TODO
class SegmentTree2D {
private:
	int n, m;
	std::vector<std::vector<int> > a, t;
	const int INF = 1 << 30;

	void build_y(int vx, int lx, int rx, int vy, int ly, int ry) {
		if (ly == ry) {
			if (lx == rx)
				t[vx][vy] = a[lx][ly];
			else
				t[vx][vy] = t[vx*2][vy] + t[vx*2+1][vy];
		} else {
			int my = (ly + ry) / 2;
			build_y(vx, lx, rx, vy*2, ly, my);
			build_y(vx, lx, rx, vy*2+1, my+1, ry);
			t[vx][vy] = t[vx][vy*2] + t[vx][vy*2+1];
		}
	}

	void build_x(int vx, int lx, int rx) {
		if (lx != rx) {
			int mx = (lx + rx) / 2;
			build_x(vx*2, lx, mx);
			build_x(vx*2+1, mx+1, rx);
		}
		build_y(vx, lx, rx, 1, 0, m-1);
	}

	int sum_y(int vx, int vy, int tly, int try_, int ly, int ry) {
		if (ly > ry) 
			return 0;
		if (ly == tly && try_ == ry)
			return t[vx][vy];
		int tmy = (tly + try_) / 2;
		return sum_y(vx, vy*2, tly, tmy, ly, min(ry, tmy))
			+ sum_y(vx, vy*2+1, tmy+1, try_, max(ly, tmy+1), ry);
	}

	int sum_x(int vx, int tlx, int trx, int lx, int rx, int ly, int ry) {
		if (lx > rx)
			return 0;
		if (lx == tlx && trx == rx)
			return sum_y(vx, 1, 0, m-1, ly, ry);
		int tmx = (tlx + trx) / 2;
		return sum_x(vx*2, tlx, tmx, lx, min(rx, tmx), ly, ry)
			+ sum_x(vx*2+1, tmx+1, trx, max(lx, tmx+1), rx, ly, ry);
	}

	void update_y(int vx, int lx, int rx, int vy, int ly, int ry, int x, int y, int new_val) {
		if (ly == ry) {
			if (lx == rx)
				t[vx][vy] = new_val;
			else
				t[vx][vy] = t[vx*2][vy] + t[vx*2+1][vy];
		} else {
			int my = (ly + ry) / 2;
			if (y <= my)
				update_y(vx, lx, rx, vy*2, ly, my, x, y, new_val);
			else
				update_y(vx, lx, rx, vy*2+1, my+1, ry, x, y, new_val);
			t[vx][vy] = t[vx][vy*2] + t[vx][vy*2+1];
		}
	}

	void update_x(int vx, int lx, int rx, int x, int y, int new_val) {
		if (lx != rx) {
			int mx = (lx + rx) / 2;
			if (x <= mx)
				update_x(vx*2, lx, mx, x, y, new_val);
			else
				update_x(vx*2+1, mx+1, rx, x, y, new_val);
		}
		update_y(vx, lx, rx, 1, 0, m-1, x, y, new_val);
	}
};






