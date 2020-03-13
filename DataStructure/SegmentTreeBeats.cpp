/** Orig https://codeforces.com/blog/entry/57319
The main idea is return whenever we can, put the tag whenever we can:

    When the operation's interval and the node's interval are no longer intersected, the information inside this subtree must not be affected. So we can return immediately.
    When the node's interval is contained by the operation's interval, all the information inside the subtree will be changed together. So we can put the tag on it and return.

And now, we can easily describe the main idea of "segment tree beats". For each node, we maintain the maximum value max_value[node] and the strict second maximum value second_value[node].
void modify(int node, int l, int r, int ll, int rr) {
  if (break_condition()) return;
  if (tag_condition()) {
      puttag(node); return;
  }
  pushdown(node);
  int mid = (l + r) >> 1;
  modify(node * 2, l, mid, ll, rr);
  modify(node * 2 + 1, mid + 1, r, ll ,rr);
  update();
}

But if we use segment tree, we can get a much simpler solution: let break_condition be l > rr || r < ll || max_value[node] < x and let tag_condition be l >= ll && r <= rr && max_value[node] == min_value[node]. And we can find that the time complexity of this segment tree is also .

*/
using ll = long long;
const int N = 1e5 + 7;
struct Node {
	ll m, s;
	Node(ll _x): m(_x), s(_x) {}
	Node() : m(0), s(0) {}
};
int n, m;
int a[N];
Node t[N << 2];

inline Node combine(const Node& lhs, const Node& rhs) {
	Node temp;
	temp.m = max(lhs.m, rhs.m);
	temp.s = lhs.s + rhs.s;
	return temp;
}

void update_2(int v, int l, int r, int x, int y, int X) {
	if (l > y || r < x || t[v].m < X) return; // break condition
	if (l == r) {
		t[v].m = t[v].s = t[v].m % X; // tag condition
		return;
	}
	int mid = (l + r) >> 1;
	update_2(v << 1, l, mid, x, y, X);
	update_2(v << 1 | 1, mid+1, r, x, y, X);
	t[v] = combine(t[v << 1], t[v << 1 | 1]);
}
void update_3(int v, int l, int r, int x, int y, int X) {
	if (l > y || r < x) return;
	if (l >= x && r <= y) {
		t[v].m = t[v].s = X;
		return;
	}
	int mid = (l + r) >> 1;
	update_3(v << 1, l, mid, x, y, X);
	update_3(v << 1 | 1, mid+1, r, x, y, X);
	t[v] = combine(t[v << 1], t[v << 1 | 1]);
}

void build(int v, int l, int r) {
	if (l > r) return;
	if (l == r) {
		t[v] = Node(a[l]);
		return;
	}
	int mid = (l + r) >> 1;
	build(v << 1, l, mid);
	build(v << 1 | 1, mid + 1, r);
	t[v] = combine(t[v << 1], t[v << 1 | 1]);
}

Node get(int v, int l, int r, int x, int y) {
	if (l > y || r < x) return Node();
	if (l >= x && r <= y) return t[v];
	int mid = (l + r) >> 1;
	return combine(get(v << 1, l, mid, x, y),
			get(v << 1 | 1, mid+1, r, x, y));
}