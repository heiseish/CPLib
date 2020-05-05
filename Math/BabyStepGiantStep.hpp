

/**
Discrete Logarithmic prblem
Baby step - giant step O(sqrt(m) * log(m))
Solve a^x≡b(mod m),
only when m is prime
*/
template <bool IsPrime>
class BabyStepGiantStep;

template <>
class BabyStepGiantStep<true> {
private:
	int a, b, m;
	int solve(int a, int b, int m) {
		int n = (int) sqrt (m + .0) + 1;

		int an = 1;
		for (int i = 0; i < n; ++i)
			an = (an * a) % m;

		std::map<int, int> vals;
		for (int p = 1, cur = an; p <= n; ++p) {
			if (!vals.count(cur))
				vals[cur] = p;
			cur = (cur * an) % m;
		}

		for (int q = 0, cur = b; q <= n; ++q) {
			if (vals.count(cur)) {
				int ans = vals[cur] * n - q;
				return ans;
			}
			cur = (cur * a) % m;
		}
		return -1;
	}
public:
	BabyStepGiantStep(int _a, int _b, int _m) : a(_a), b(_b), m(_m) {};
	int solve() {
		return solve(a, b, m);
	}
}

template <>
class BabyStepGiantStep<false> {
private:
	int a, b, m;
	int powmod(long long x, long long p, long long m) {
		long long v = 1;
		while(p) {
			if(p & 1) v = x * v % m;
			x = x * x % m;
			p >>= 1;
		}

		return v;
	}

	int gcd(int a, int b) {
		while(b) {
			a %= b;
			swap(a, b);
		}
		return a;
	}

	int solve(int a, int b, int p) {
		a %= p, b %= p;
		if(b == 1) return 0;
		int cnt = 0;
		long long t = 1;
		for(int g = gcd(a, p); g != 1; g = gcd(a, p)) {
			if(b % g) return -1;
			p /= g, b /= g, t = t * a / g % p;
			++cnt;
			if(b == t) return cnt;
		}

		map<int, int> vals;
		int m = int(sqrt(1.0 * p) + 1);
		long long base = b;
		for(int i = 0; i != m; ++i) {
			vals[base] = i;
			base = base * a % p;
		}

		base = powmod(a, m, p);
		long long now = t;
		for(int i = 1; i <= m + 1; ++i) {
			now = now * base % p;
			if(vals.count(now))
				return i * m - vals[now] + cnt;
		}

		return -1;
	}

public:
	BabyStepGiantStep(int _a, int _b, int _m) : a(_a), b(_b), m(_m) { }
	int solve(){
		return solve(a, b, m);
	}
};
