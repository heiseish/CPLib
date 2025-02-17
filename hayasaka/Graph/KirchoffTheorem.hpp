#include <vector>
class KirchoffTheorem {
private:
	std::vector<std::vector<double> > AdjMat, DegreeMat;
	int N;
	long long determinant(std::vector<std::vector<double> >& a) {
		const double EPS = 1E-9;
		int n = a.size();
		double det = 1;
		for (int i=0; i<n; ++i) {
			int k = i;
			for (int j=i+1; j<n; ++j)
				if (abs (a[j][i]) > abs (a[k][i]))
					k = j;
			if (abs (a[k][i]) < EPS) {
				det = 0;
				break;
			}
			swap (a[i], a[k]);
			if (i != k)
				det = -det;
			det *= a[i][i];
			for (int j=i+1; j<n; ++j)
				a[i][j] /= a[i][i];
			for (int j=0; j<n; ++j)
				if (j != i && abs (a[j][i]) > EPS)
					for (int k=i+1; k<n; ++k)
						a[j][k] -= a[i][k] * a[j][i];
		}

		return (long long) det;
	}
public:
	// 0-based
	KirchoffTheorem(const std::vector<std::vector<int> >& AdjList) : N(AdjList.size()) {
		AdjMat.assign(N, std::vector<double>(N, 0.0));
		DegreeMat.assign(N, std::vector<double>(N, 0.0));
		for(int i = 0; i < N; ++i) {
			for (auto &v : AdjList[i]) {
				AdjMat[i][v]++;
				AdjMat[v][i]++;
			}
			DegreeMat[i][i] = AdjList[i].size();
		}
	}

	long long number_of_Spanning_Tree() {
		std::vector<std::vector<double> > res(N, std::vector<double>(N));
		for(int i =0; i < N; ++i) {
			for(int j = 0; j < N; ++j) {
				res[i][j] = DegreeMat[i][j] - AdjMat[i][j];
			}
		}
		return determinant(res);
	}
};