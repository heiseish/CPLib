#include <vector>
/**
Matrix multiplication
*/
struct MatrixMul {
	int MOD = 1e7 + 9;
	std::vector<std::vector<long long> > matrixMulty(const std::vector<std::vector<long long> > &a, const std::vector<std::vector<long long> > &b) {
		int m=a.size(), n=a[0].size(), p = b[0].size();
		std::vector<std::vector<long long> > res(m, std::vector<long long> (p, 0));
		for(int i=0;i<=m-1;++i) for(int j=0;j<=n-1;++j) for(int k=0;k<=p-1;++k)
		res[i][k]=properMod(res[i][k]+1LL*a[i][j]*b[j][k], MOD);
		return res;
	}
	std::vector<std::vector<long long> > matrixUnit(int n) {
		std::vector<std::vector<long long> > res(n, std::vector<long long> (n, 0));
		for(int i=0;i<=n-1;++i) res[i][i]=1;
		return res;
	}
	std::vector<std::vector<long long> > matrixPower(std::vector<std::vector<long long> > a, long long n) {
		std::vector<std::vector<long long> > res = matrixUnit(a.size());
		for(;n>0;n>>=1) {
			if (n&1) res=matrixMulty(res, a);
			a=matrixMulty(a, a);
		}
		return res;
	}

		// example
	long long Fibonacci(int n) {
		std::vector<std::vector<long long> > base(2, std::vector<long long> (2, 1));
		base[0][0]=0;
		std::vector<std::vector<long long> > First(2, std::vector<long long> (1, 1));
		return matrixMulty(First, matrixPower(base, n+1))[1][0];
	}

	struct AugmentedMatrix { double mat[MAX_N][MAX_N + 1]; };
	struct ColumnVector { double vec[MAX_N]; };

	ColumnVector GaussianElimination(int N, AugmentedMatrix Aug) {
	  // input: N, Augmented Matrix Aug, output: Column vector X, the answer
		int i, j, k, l; double t;

	  for (i = 0; i < N - 1; i++) {            // the forward elimination phase
	  	l = i;
	    for (j = i + 1; j < N; j++)       // which row has largest column value
	    if (fabs(Aug.mat[j][i]) > fabs(Aug.mat[l][i]))
	        l = j;                                       // remember this row l
	    // swap this pivot row, reason: minimize floating point error
	    for (k = i; k <= N; k++)            // t is a temporary double variable
	    t = Aug.mat[i][k], Aug.mat[i][k] = Aug.mat[l][k], Aug.mat[l][k] = t;
	    for (j = i + 1; j < N; j++)     // the actual forward elimination phase
	    for (k = N; k >= i; k--)
	    Aug.mat[j][k] -= Aug.mat[i][k] * Aug.mat[j][i] / Aug.mat[i][i];
	}

	  ColumnVector Ans;                          // the back substitution phase
	  for (j = N - 1; j >= 0; j--) {                         // start from back
	  	for (t = 0.0, k = j + 1; k < N; k++) t += Aug.mat[j][k] * Ans.vec[k];
	    Ans.vec[j] = (Aug.mat[j][N] - t) / Aug.mat[j][j]; // the answer is here
	}
	return Ans;
	}
}