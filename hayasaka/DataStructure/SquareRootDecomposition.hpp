#include <vector>
#include <cmath>
// Calculate range sum in O(sqrt(n))
class SqrtDecompositionSum {
private:
	int N;
	int len;
	std::vector<int> a, b;
public:
	SqrtDecompositionSum(const std::vector<int>& _a) : a(_a), N(_a.size()) {
		len = (int) sqrt (n + .0) + 1; // size of the block and the number of blocks
		b.resize(n);
		for (int i=0; i<n; ++i)
			b[i / len] += a[i];
	}

	void update(int idx, int new_val) {
		b[idx / len] += new_val - a[idx];
		a[idx] = new_val;
	}
	
	int query(int l, int r) {
		int sum = 0;
		int c_l = l / len,   c_r = r / len;
		if (c_l == c_r)
			for (int i=l; i<=r; ++i)
				sum += a[i];
		else {
			for (int i=l, end=(c_l+1)*len-1; i<=end; ++i)
				sum += a[i];
			for (int i=c_l+1; i<=c_r-1; ++i)
				sum += b[i];
			for (int i=c_r*len; i<=r; ++i)
				sum += a[i];
		}
		return sum;
	}
};