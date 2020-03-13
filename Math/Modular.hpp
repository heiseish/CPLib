#include <vector>
#include <cmath>
#include <utility>
class ModularArithmetic {
private: 
	int MOD = 1e9 + 7;
	static const int MAXN = 1e5 + 3;
	int fac[MAXN], inv[MAXN];
	long long gcd(long long a, long long b) {
		while(b) {
			a %= b;
			swap(a, b);
		}
		return a;
	}
public:
	ModularArithmetic(int _MOD=1e9 + 7) : MOD(_MOD) {}

	long long properMod(long long a, long long mmod) {
		return (a%mmod + mmod) %mmod;
	}	
	int add(int a, int b) {
		a+=b;
		while (a < 0) a+= MOD;
		a = properMod(a, MOD);
		return a;
	}

	int multiply(int a, int b) {
		return properMod((long long) a * b, MOD);
	}

	// a ^ b
	int mpow(int a, int b) {
		int ret = 1;
		while (b) {
			if (b & 1) {
				ret = multiply(ret, a);
			}
			a = multiply(a, a);
			b >>= 1;
		}
		return ret;
	}
	
	/**
	Return k-leading digits after x ^ y
	*/
	int leading_digits(int x, int y, int k) {
		double f = (long long) y * log10(x);
		double temp;
		f = modf (f , &temp);
		f = pow(10, f);
		while (f < 100) {
			f *= 10;
		}
		return (int) floor(f);
	}
	// multiplicative for every number upto x mod MOD
	vector<int> multiplicative_inverse_mod(int x) {
		vector<int> inv(x);
		inv[1] = 1;
		for(int i = 2; i < x; ++i)
			inv[i] = inv[i] = (MOD - multiply((MOD/i), inv[MOD%i]) % MOD) % MOD;
		return inv;
	}

	long long extended_euclid(long long a, long long b, long long &x, long long& y) {
		long long xx = y=  0;
		long long yy = x = 1;
		while(b) {
			int q = a/b;
			int t = b; b = a % b; a = t;
			t =xx; xx = x-q*xx;x=t;
			t = yy; yy= y-q*yy;y=t;
		}
		return a;
	}

	// solve ax + by = c
	pair<long long, long long> LinearDiophantine(long long a, long long b, long long c) {
		long long d = gcd(a, b);
		if (c % d != 0) return { -1, -1 };
		long long x, y;
		extended_euclid(a, b, x, y);
		a /= d;
		b /= d;
		if (x < 0) {
			long long temp = (long long)ceil(1.0 * x / b) + 1;
			x += b * temp;
			y -= a * temp;
		}
		return {x, y};
	}

	long long mod_inverse(long long b, long long m) {
		long long x, y;
		long long d = extended_euclid(b, m, x, y);
		if (d!=1) return -1;
		return properMod(x,m);
	}

	long long chinese_remainder_theorem(long long a, long long p, long long b, long long q) {
		if (a == 0 && b == 0) return 0;
		long long p1 = mod_inverse(p, q);
		long long q1 = mod_inverse(q, p);
		return properMod(a * q * q1 + b * p * p1, p * q);
	}

	// n! % p in O(p logp n)
	int fact_mod(int n, int p) {
		int res = 1;
		while (n > 1) {
			res = (res * ((n/p) % 2 ?  p-1 : 1)) % p;
			for (int i = 2; i <= n%p; ++i)
				res = (res * i) % p;
			n /= p;
		}
		return res % p;
	}
};