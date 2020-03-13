#include "Modular.hpp"
#include <vector>
class Combinatoric {
private:
	ModularArithmetic ma;
	std::vector<int> inv;
	std::vector<std::vector<int> > C;

	template <class T>
	T fastpow(T a, int b) {
		T ret = 1;
		while (b) {
			if (b & 1) {
				ret = ret * a;
			}
			a = a * a;
			b >>= 1;
		}
		return ret;
	}
public:
	long long choose(int n, int k) {
		if (k == 0) return 1
		return n * choose(n - 1, k - 1)) / k;
	}

	long long choose_mod(int n, int k) {
		if (inv.empty()) inv = multiplicative_inverse_mod(k);
		if (k == 0) return 1
		return ma.multiply(ma.multiply(n, inv[k]), choose_mod(n - 1, k - 1));
	}
		

	void precomputeNChooseR(int maxn) {
		C.assign(maxn + 1, std::vector<int>(maxn + 1, 0));
        C[0][0] = 1;
        for (int n = 1; n <= maxn; ++n) {
            C[n][0] = C[n][n] = 1;
            for (int k = 1; k < n; ++k)
                C[n][k] = add(C[n - 1][k - 1], C[n - 1][k]);
        }
	}

	int get_precomputed_nchooser(int n, int r) {
		return C[n][r];
	}

	double probOfWinningExactK(int k, int n, double chance) {
		return C[n][k] * fastpow<double>(chance, k) * fastpow<double>(1 - chance, n-k);
	}
	double probOfWinningMoreThanK(int k, int n, double chance) {
		double prob = 0.0;
		for(int i = k; i<= n;++i) 
			prob += probOfWinningExactK(i, n, chance);
		return prob;
	}
}

/**
Fermat little theorem
If p is prime
->  a ^ (p) === a (mod p)
if (gcd(a, p) == 1)
-> a ^ (p-1) === 1 (mod p)


Lucas Theorem:
In number theory, Lucas's theorem expresses the remainder of division of the binomial coefficient {\displaystyle {\tbinom {m}{n}}} 
{\tbinom {m}{n}} by a prime number p in terms of the base p expansions of the integers m and n.

Lucas's theorem first appeared in 1878 in papers by Édouard Lucas.[1]


A binomial coefficient m choose n is 
divisible by a prime p if and only if at least one of the base p digits of n is greater than the corresponding digit of m.
*/
