
/**
In modular arithmetic, a number g is called a primitive root modulo n 
if every number coprime to n is congruent to a power of g modulo n. 
Mathematically, g is a primitive root modulo n if and only if for 
any integer a such that gcd(a,n)=1, there exists an integer k such 
that:

gk≡a(modn).

k is then called the index or discrete logarithm of a to the base 
g modulo n. g is also called the generator of the multiplicative 
group of integers modulo n.
*/
class PrimitiveRoot {
private:
	int powmod (int a, int b, int p) {
		int res = 1;
		while (b)
			if (b & 1)
				res = int (res * 1ll * a % p),  --b;
			else
				a = int (a * 1ll * a % p),  b >>= 1;
		return res;
	}
public:
	int generator (int p) {
		std::vector<int> fact;
		int phi = p-1,  n = phi;
		for (int i=2; i*i<=n; ++i)
			if (n % i == 0) {
				fact.push_back (i);
				while (n % i == 0)
					n /= i;
			}
		if (n > 1)
			fact.push_back (n);

		for (int res=2; res<=p; ++res) {
			bool ok = true;
			for (size_t i=0; i<fact.size() && ok; ++i)
				ok &= powmod (res, phi / fact[i], p) != 1;
			if (ok)  return res;
		}
		return -1;
	}
}
