#include <vector>
/**
N' = 2 ^ ceil(ln(N))
O(1) query
O(N' ln N') preprocessing
https://discuss.codechef.com/t/tutorial-disjoint-sparse-table/17404
*/
class DST {
private:
	static const int maxn = 1e6 + 5;
	static const int maxpown = 1048576; // 2^(ceil(log_2(maxn)))
	static const int MAXLEV = 21;// ceil(log_2(maxn)) + 1
	int N, P;
	std::vector<int> A;
	std::vector<std::vector<int> > t;
	int maxlev, size;

	void build() {
		int n = 1; while(n < N) n <<= 1;
		int range, half;
		for(int h = 1; (range = 1 <> 1;
			for(int i = half; i < n; i += range) {
				t[h][i - 1] = A[i - 1];
				for(int j = i - 2; j >= i - half; j--)
					t[h][j] = t[h][j + 1] * A[j] % P;
				t[h] = A;
				for(int j = i + 1; j < i + half; j++)
					t[h][j] = t[h][j - 1] * A[j] % P;
			}
		}
	}
public:
	DFS(const vector<int>& _A, int _P) : A(_A), N(_A.size()), P(_P) {
		table.assign(MAXLEV, std::vector<int>(maxpown, 0));
		size = N;
		maxlev = __builtin_clz(n) ^ 31;		// floor(log_2(n))
		if( (1<<maxlev) != n)
			size = 1<<++maxlev;
		build()
	}

	// Range product query mod P
	int query(int L, int R) {
		int result;
		if(L == R) result = A[L];
		else {
			int h = sizeof(int) * __CHAR_BIT__ - __builtin_clz(L ^ R);
			result = t[h][L] * t[h][R] % P;
		}
		return result;
	}
};