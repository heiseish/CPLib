#include <utility>
// sum of Floors of [p/q] + [2p/q] + .... + [np/q]
long long sum_of_floors(long long p, long long q, long long n) {
	long long t = gcd(p, q);
	p /= t;
	q /= t;
	long long s = 0;
	long long z = 1;
	while(q > 0 && n > 0) {
		t = p/q;
		s += z*t*n * (n + 1) /2;
		p -= q * t;

		t = n/q;
		s += z * p * t * (n + 1) - z * t * (p * q * t + p + q - 1)/2;
		n -= q * t;

		t = (n * p)/ q;
		s += z * t * n;
		n = t;
		swap(p, q);
		z = -z;
	}
	return s;
}

int gcd(int a, int b) {
	while(b) {
		a %= b;
		swap(a, b);
	}
	return a;
}

int lcm(int a, int b) {
	return a * (b / gcd(a, b));
}