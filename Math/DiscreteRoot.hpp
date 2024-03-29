#include <vector>
#include <algorithm>
#include <cmath>
#include <utility>

class DiscreteRoot {
private:
	int gcd(int a, int b) {
		while(b) {
			a %= b;
			swap(a, b);
		}
		return a;
	}

	int powmod(int a, int b, int p) {
		int res = 1;
		while (b > 0) {
			if (b & 1) {
				res = res * a % p;
			}
			a = a * a % p;
			b >>= 1;
		}
		return res;
	}

	// Finds the primitive root modulo p
	int generator(int p) {
		std::vector<int> fact;
		int phi = p-1, n = phi;
		for (int i = 2; i * i <= n; ++i) {
			if (n % i == 0) {
				fact.push_back(i);
				while (n % i == 0)
					n /= i;
			}
		}
		if (n > 1)
			fact.push_back(n);

		for (int res = 2; res <= p; ++res) {
			bool ok = true;
			for (int factor : fact) {
				if (powmod(res, phi / factor, p) == 1) {
					ok = false;
					break;
				}
			}
			if (ok) return res;
		}
		return -1;
	}
public:
	// This program finds all numbers x such that x^k = a (mod n)
	std::vector<int> solve(int n, int k, int a) {
		int g = generator(n);

		// Baby-step giant-step discrete logarithm algorithm
		int sq = (int) sqrt (n + .0) + 1;
		std::vector<std::pair<int, int> > dec(sq);
		for (int i = 1; i <= sq; ++i)
			dec[i-1] = {powmod(g, i * sq * k % (n - 1), n), i};
		std::sort(dec.begin(), dec.end());
		int any_ans = -1;
		for (int i = 0; i < sq; ++i) {
			int my = powmod(g, i * k % (n - 1), n) * a % n;
			auto it = std::lower_bound(dec.begin(), dec.end(), std::make_pair(my, 0));
			if (it != dec.end() && it->first == my) {
				any_ans = it->second * sq - i;
				break;
			}
		}
		if (any_ans == -1) return { }

		// Print all possible answers
		int delta = (n-1) / gcd(k, n-1);
		vector<int> ans;
		for (int cur = any_ans % delta; cur < n-1; cur += delta)
			ans.push_back(powmod(g, cur, n));
		std::sort(ans.begin(), ans.end());
		return ans;
	}
}