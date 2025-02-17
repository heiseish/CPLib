#include <std::vector>
#include <algorithm>
#include <utility>

class UnionFind {
private:
	int n;
	std::vector<int> gr, rank;
public:
	UnionFind(int _n) : n(_n) {
		gr.assign(n, -1);
		rank.assign(n, 1);
	}

	int find(int index) {
		if (gr[index] == -1) return index;
		return gr[index] = find(gr[index]);
	}

	bool join(int a, int b) {
		a = find(a);
		b=  find(b);
		if (a == b) return false;
		if (rank[a] < rank[b]) 
			swap(a, b);
		gr[b] = a;
		if (rank[a] == rank[b])
            rank[a]++;
		return true;	
	}
};

class DSUWithBipartiteChecking {
private:
	int N;
	std::vector<int> parent, rank, bipartite;
	void make_set(int v) {
		parent[v] = make_pair(v, 0);
		rank[v] = 0;
		bipartite[v] = true;
	}
public:
	DSUWithBipartiteChecking(int _N) : N(_N) {
		parent.resize(N);
		rank.resize(N);
		bipartite.resize(N);
		for(int i = 0; i < N; ++i) make_set(i);
	}
	std::pair<int, int> find_set(int v) {
		if (v != parent[v].first) {
			int parity = parent[v].second;
			parent[v] = find_set(parent[v].first);
			parent[v].second ^= parity;
		}
		return parent[v];
	}

	void add_edge(int a, int b) {
		std::pair<int, int> pa = find_set(a);
		a = pa.first;
		int x = pa.second;

		pair<int, int> pb = find_set(b);
		b = pb.first;
		int y = pb.second;

		if (a == b) {
			if (x == y)
				bipartite[a] = false;
		} else {
			if (rank[a] < rank[b])
				swap (a, b);
			parent[b] = make_pair(a, x^y^1);
			bipartite[a] &= bipartite[b];
			if (rank[a] == rank[b])
				++rank[a];
		}
	}

	bool is_bipartite(int v) {
		return bipartite[find_set(v).first];
	}
};

/**
Offline RMQ (range minimum query) in O(α(n)) on average / Arpa's trick
We are given an array a[] and we have to compute some minima in given segments of the array.

The idea to solve this problem with DSU is the following: We will iterate over 
the array and when we are at the ith element we will answer all queries (L, R) with R == i. 
To do this efficiently we will keep a DSU using the first i elements with the following 
structure: the parent of an element is the next smaller element to the right of it. 
Then using this structure the answer to a query will be the a[find_set(L)], the smallest number 
to the right of L.
*/
// INCOMPLETE
class DSUAprpaTrick {
private:
	struct Query {
		int L, R, idx;
	};
	int N;
	std::vector<int> answer;
	std::vector<std::vector<Query> > container; //container[i] contains all queries with R == i.
public:
	DSUAprpaTrick(const std::vector<std::vector<Query> > input, int num_question) : container(input), N(num_question) {
		answer.resize(num_question);
	}
	void solve() {
		stack<int> s;
		for (int i = 0; i < n; i++) {
			while (!s.empty() && a[s.top()] > a[i]) {
				parent[s.top()] = i;
				s.pop();
			}
			s.push(i);
			for (Query q : container[i]) {
				answer[q.idx] = a[find_set(q.L)];
			}
		}
	}
};
