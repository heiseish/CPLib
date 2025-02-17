const int N = 1e5 + 7;
//https://codeforces.com/blog/entry/53925
//https://codeforces.com/blog/entry/54150
//https://artofproblemsolving.com/community/c90633h1291397
int eulerphi[N], primechk[N], mu[N];
// precal in O(n log log n)
auto precalc(const int n) -> void {
    int i, j;
    eulerphi[1] = 1; mu[1] = 1;
    for (i = 2; i <= n; ++i) {
        eulerphi[i] = i;
        primechk[i] = 1;
        mu[i] = 1;
    }
    for (i = 2; i <= n; ++i) {
        if (primechk[i] == 1) {
            mu[i] = -mu[i];
            eulerphi[i] -= eulerphi[i] / i;
            for (j = 2; i * j <= n ; ++j) {
                primechk[i*j] = 0;
                eulerphi[i*j] -= eulerphi[i*j] / i;

                if (j % i == 0) mu[i*j] = 0;  
                else mu[i*j] = -mu[i*j];
            }
        }
    }
}

// Number of coprime pair (x, y) in range [1, n];
// precalc O(n lg lg n)
// loop O(n)
auto f(int n) -> long long {
    precalc(n);
    long long ans = 0LL;
    for (int d = 1; d <= n; ++d) {
        ans += mu[d] * (n / d) * (n /d);
    }
    return ans;
}


/// loop through harmonic sequence floor(n/1) + floor(n/2) + ... + floor(n/n) in
// O(sqrt(n))
auto g(int n) -> void {
    for (int i = 1, la; i <= n; i = la + 1) {
        la = n / (n / i);
        //n / x yields the same value for i <= x <= la.
    }
}

// Sum of gcd(a, b) for all pair (a, b) in range [1, n]
// O(n lg lg n)
auto h(int n) -> long long {
    precalc(n);
    long long ans = 0LL;
    for (int l = 1; l <= n; ++l) {
        ans += (n/l) * (n/l) * eulerphi[l];
    }
    return ans;
}