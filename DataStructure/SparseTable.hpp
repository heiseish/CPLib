#include <vector>
#include <cmath>
#include <algorithm>

class SparseTableSum {
private:
	int N, K;
	std::vector<std::vector<long long> > st;
public:
	// O(n log n) build
	SparseTableSum(vector<int>& a) : N(a.size()), K(log2(a.size()) + 1) {
		st.assign(N + 1, std::vector<int>(K + 1, 0LL));
		for (int i = 0; i < N; i++)
			st[i][0] = a[i];

		for (int j = 1; j <= K; j++)
			for (int i = 0; i + (1 << j) <= N; i++)
				st[i][j] = st[i][j-1] + st[i + (1 << (j - 1))][j - 1];
	}

	// binary lifting
	long long query(int L, int R) {
		long long sum = 0LL;
		for (int j = K; j >= 0; j--) {
			if ((1 << j) <= R - L + 1) {
				sum += st[L][j];
				L += 1 << j;
			}
		}
		return sum;
	}
};

class SparseTableMin {
private:
	int N, K;
	std::vector<std::vector<long long> > st;
	std::vector<int> lg;
public:
	SparseTableMin(vector<int>& a) : N(a.size()), K(log2(a.size()) + 1) {
		st.assign(N + 1, std::vector<int>(K + 1, 0LL));
		lg.assign(N + 1, 0);
		for (int i = 2; i <= N; i++)
			lg[i] = lg[i/2] + 1;

		for (int i = 0; i < N; i++)
			st[i][0] = a[i];

		for (int j = 1; j <= K; j++)
			for (int i = 0; i + (1 << j) <= N; i++)
				st[i][j] = min(st[i][j-1], st[i + (1 << (j - 1))][j - 1]);
	}

	long long query(int L, int R) {
		int j = lg[R - L + 1];
		return min(st[L][j], st[R - (1 << j) + 1][j]);
	}
};