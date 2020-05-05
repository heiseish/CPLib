#include "Modular.hpp"
#include <vector>


const int N = 2e5 + 7;
int f[N], inv[N], ans[N];
const int MOD = 1e9 + 7;

int add(int a, int b) {
    a += b;
    a %= MOD;
    if (a < MOD) a += MOD;
    return a % MOD;
}
int mul(int a, int b) {
    return (long long) a * b % MOD;
}

int bin_pow(int a, int b) {
    a %= MOD;
    int ret = 1;
    while (b) {
        if (b & 1) ret = mul(ret, a);
        a = mul(a, a);
        b >>= 1;
    }
    return ret;
}

auto precal_inv() -> void {
    inv[1] = 1;
	for(int i = 2; i <= n; ++i)
		inv[i] = add(MOD, -mul((MOD/i), inv[MOD%i]));
}
int choose(int n, int k) {
	if (k == 0) return 1;
	return mul(mul(n, inv[k]), choose(n - 1, k - 1));
}

// O(k lg N) Stirling number of second kind
int StirlingSecond(int n, int k) {
	int  ret = 0, com = 1;
	for (int i = 0; i <= k; ++i) {
		int ss = (i & 1) ? -1 : 1;
		ret = add(ret, mul(ss, mul(com, bin_pow(k - i, n))));
		com = mul(com, mul(k-i, inv[i+1]));
	}
	return mul(ret, bin_pow(f[k], MOD - 2));
}

long long choose(int n, int k) {
    if (k == 0) return 1
    return n * choose(n - 1, k - 1)) / k;
}

int choose_mod(int n, int k) {
    if (inv.empty()) inv = multiplicative_inverse_mod(k);
    if (k == 0) return 1
    return ma.multiply(ma.multiply(n, inv[k]), choose_mod(n - 1, k - 1));
}


const int MAXN = 2e3 + 7;
int C[MAXN][MAXN];
void precomputeNChooseR(int maxn) {
    C[0][0] = 1;
    for (int i = 1; i <= 100; ++i) {
        C[i][0] = C[i][i] = 1;
        for (int j = 1; j < i; ++j)
            C[i][j] = add(C[i - 1][j - 1], C[i - 1][j]);
    }
}

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

double probOfWinningExactK(int k, int n, double chance) {
    return C[n][k] * fastpow<double>(chance, k) * fastpow<double>(1 - chance, n-k);
}

double probOfWinningMoreThanK(int k, int n, double chance) {
    double prob = 0.0;
    for(int i = k; i<= n;++i) 
        prob += probOfWinningExactK(i, n, chance);
    return prob;
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

int gcd(int a, int b) {
    while (b) {
        a %= b;
        swap(a, b);
    }
    return a;
}

/**
Return k th-leading digits after x ^ y
*/
int leading_digits(int x, int y, int k) {
    double f = (long long) y * log10(x);
    double temp;
    f = modf (f , &temp); // get fractional part
    f = pow(10, f);
    while (f < 100) {
        f *= 10;
    }
    return (int) floor(f);
}

int extended_euclid(int a, int b, int &x, int& y) {
    int xx = y=  0;
    int yy = x = 1;
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
    if (d != 1) return -1;
    return add(x, 0);
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


template <class T>
T order_statistics (std::vector<T> a, unsigned n, unsigned k) {
    using std::swap;
    for (unsigned l=1, r=n; ; ) {
        if (r <= l+1) {
            // the current part size is either 1 or 2, so it is easy to find the answer
            if (r == l+1 && a[r] < a[l])
                swap (a[l], a[r]);
            return a[k];
        }

        // ordering a[l], a[l+1], a[r]
        unsigned mid = (l + r) >> 1;
        swap (a[mid], a[l+1]);
        if (a[l] > a[r])
            swap (a[l], a[r]);
        if (a[l+1] > a[r])
            swap (a[l+1], a[r]);
        if (a[l] > a[l+1])
            swap (a[l], a[l+1]);

        // performing division
        // barrier is a[l + 1], i.e. median among a[l], a[l + 1], a[r]
        unsigned i = l+1, j = r;
        const T cur = a[l+1];
        for (;;) {
            while (a[++i] < cur) ;
            while (a[--j] > cur) ;
            if (i > j)
                break;
            swap (a[i], a[j]);
        }
        // inserting the barrier
        a[l+1] = a[j];
        a[j] = cur;
        // we continue to work in that part, which must contain the required element
        if (j >= k)
            r = j-1;
        if (j <= k)
            l = i;
    }
}



