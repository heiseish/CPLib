#include <bits/stdc++.h>
#define PI acos(-1)

using namespace std;

class ColleyTukey {
private:
	int reverse(int num, int lg_n) {
		int res = 0;
		for (int i = 0; i < lg_n; i++) {
			if (num & (1 << i))
				res |= 1 << (lg_n - 1 - i);
		}
		return res;
	}


	void fft(vector<cd> & a, bool invert) {
		using cd = complex<double>;
		int n = a.size();

		for (int i = 1, j = 0; i < n; i++) {
			int bit = n >> 1;
			for (; j & bit; bit >>= 1)
				j ^= bit;
			j ^= bit;

			if (i < j)
				swap(a[i], a[j]);
		}

		for (int len = 2; len <= n; len <<= 1) {
			double ang = 2 * PI / len * (invert ? -1 : 1);
			cd wlen(cos(ang), sin(ang));
			for (int i = 0; i < n; i += len) {
				cd w(1);
				for (int j = 0; j < len / 2; j++) {
					cd u = a[i+j], v = a[i+j+len/2] * w;
					a[i+j] = u + v;
					a[i+j+len/2] = u - v;
					w *= wlen;
				}
			}
		}

		if (invert) {
			for (cd & x : a)
				x /= n;
		}
	}
public:
	vector<int> multiply(vector<int> const& a, vector<int> const& b) {
		vector<cd> fa(a.begin(), a.end()), fb(b.begin(), b.end());
		int n = 1;
		while (n < a.size() + b.size()) 
			n <<= 1;
		fa.resize(n);
		fb.resize(n);

		fft(fa, false);
		fft(fb, false);
		for (int i = 0; i < n; i++)
			fa[i] *= fb[i];
		fft(fa, true);

		vector<int> result(n);
		for (int i = 0; i < n; i++)
			result[i] = round(fa[i].real());
		return result;
	}
}


class ColleyTukeyMod {
private:
	static const int mod = 7340033;
	static const int root = 5;
	static const int root_1 = 4404020;
	static const int root_pw = 1 << 20;
public:
	void fft(vector<int> & a, bool invert) {
		int n = a.size();

		for (int i = 1, j = 0; i < n; i++) {
			int bit = n >> 1;
			for (; j & bit; bit >>= 1)
				j ^= bit;
			j ^= bit;
			if (i < j)
				swap(a[i], a[j]);
		}
		for (int len = 2; len <= n; len <<= 1) {
			int wlen = invert ? root_1 : root;
			for (int i = len; i < root_pw; i <<= 1)
				wlen = (int)(1LL * wlen * wlen % mod);
			for (int i = 0; i < n; i += len) {
				int w = 1;
				for (int j = 0; j < len / 2; j++) {
					int u = a[i+j], v = (int)(1LL * a[i+j+len/2] * w % mod);
					a[i+j] = u + v < mod ? u + v : u + v - mod;
					a[i+j+len/2] = u - v >= 0 ? u - v : u - v + mod;
					w = (int)(1LL * w * wlen % mod);
				}
			}
		}
		if (invert) {
			int n_1 = inverse(n, mod);
			for (int & x : a)
				x = (int)(1LL * x * n_1 % mod);
		}
	}
}


/**
- Multiply 2 longs numbers
int carry = 0;
for (int i = 0; i < n; i++)
	result[i] += carry;
	carry = result[i] / 10;
	result[i] %= 10;
}

- All possible scalar product
We are given two arrays a[] and b[] of length n. We have to compute the 
products of a with every cyclic shift of b.

We generate two new arrays of size 2n: We reverse a and append n zeros to it. 
And we just append b to itself. When we multiply these two arrays as polynomials, and look at the coefficient c[n−1], c[n], c[2n−2] of the product c, we get:
c[k]=∑i+j=ka[i]b[j]
And since all the elements a[i]=0 for i≥n:
c[k]=∑i=0n−1a[i]b[k−i]
It is easy to see that this sum is just the scalar product of the vector a 
with the (k−(n−1))-th cyclic left shift of b. Thus these coefficients are the 
answer to the problem, and we were still able to obtain it in O(nlogn) time. 
Note here that c[2n−1] also gives us the n-th cyclic shift but that is the 
same as the 0-th cyclic shift so we don't need to consider that separately into our answer.
*/