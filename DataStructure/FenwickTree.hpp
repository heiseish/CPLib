#include <vector>
#define LSOne(S) ((S) & -(S))

class FenwickTree {
private: 
	std::vector<int> ft; 
	int n;       
public: 
	FenwickTree(int _n) : n(_n) { ft.assign(n+1, 0); }    
	FenwickTree(const std::vector<int>& f) : n(f.size()-1) { ft.assign(n+1, 0);
    	for (int i = 1; i <= n; i++) {                      
    		ft[i] += f[i];                              
    		if (i+LSOne(i) <= n)  ft[i+LSOne(i)] += ft[i]; 
		} 
	}     
	int rsq(int j) {                         
   		int sum = 0; 
		for (; j; j -= LSOne(j)) sum += ft[j];
    	return sum; 
	}
	int rsq(int i, int j) { 
		return rsq(j) - rsq(i-1); 
	} 
  	void update(int i, int v) {
    	for (; i <= n; i += LSOne(i)) ft[i] += v; 
	} 
	int select(int k) {
    	int lo = 1, hi = n;
    	for (int i = 0; i < 30; i++) { 
      		int mid = (lo+hi) / 2;             
      		(rsq(1, mid) < k) ? lo = mid : hi = mid; 
		}
    	return hi; 
	}
};

class RUPQ : FenwickTree {  
public:
  RUPQ(int n) : FenwickTree(n) {}
  int point_query(int i) { return rsq(i); }
  void range_update(int i, int j, int v) { update(i, v), update(j+1, -v); }
};

class RURQ {
private: 
	std::vector<long, long> fta, ftb; int n;  
public:
	RURQ(int _n) : n(_n) {
		fta.assign(n+1, 0);
		ftb.assign(n+1, 0);
	}
	long long queryA(int b)	{
		long long sum = 0long long;
		for (; b; b -= LSOne(b)) sum += fta[b];
		return sum;
	}

	long long queryB(int b)	{
		long long sum = 0long long;
		for (; b; b -= LSOne(b)) sum += ftb[b];
		return sum;
	}

	long long query(int b) {
		return queryA(b) * b - queryB(b);
	}

	long long query(int i, int j) {
		return query(j) - query(i - 1);
	}

	// Point update: Adds v to the value at position k in the array
	// ft is the fenwick tree which represents that array
	void updateA(int k, long long v) {
		for (; k <= n; k += LSOne(k)) fta[k] += v;
	}

	void updateB(int k, long long v) {
		for (; k <= n; k += LSOne(k)) ftb[k] += v;
	}

	// Range update: Adds v to each element in [i...j]
	void range_update(int i, int j, long long v)	{
		updateA(i, v);
		updateA(j + 1, -v);
		updateB(i, v * (i - 1));
		updateB(j + 1, -v * j);
	}
};

struct FenwickTreeMin {
    std::vector<int> bit;
    int n;
    const int INF = 1e9;

    FenwickTreeMin(int n): n(n) {
        bit.assign(n + 1, INF);
    }

    FenwickTreeMin(std::vector<int> a) : FenwickTreeMin(a.size()-1) {
        for (size_t i = 1; i <= n; i++)
            update(i, a[i]);
    }

    int query(int r) {
        int ret = INF;
        for (; r; r -=LSOne(r))
            ret = min(ret, bit[r]);
        return ret;
    }

    void update(int idx, int val) {
        for (; idx <= n; idx +=LSOne(idx))
            bit[idx] = min(bit[idx], val);
    }
};

class FenwickTree2D {                  
private: 
	std::vector<std::vector<int> > ft; 
	int n, m;  
public: 
	FenwickTree2D(int _n, int _m) : n(_n), m(_m) { 
		ft.assign(n+1, std::vector<int>(m+1, 0)); 
	}    
	void update(int i, int j, int v) {
    	for (int x=i; x <= n; x += LSOne(x)) 
			for(int y=j;y<=m;y+=LSOne(y)) 
				ft[x][y] += v; 
	}    

	FenwickTree2D(const std::vector<std::vector<int> >& f) : n(f.size()-1), m(f.back().size() - 1) { 
		ft.assign(n+1, std::vector<int>(m+1, 0));
		for (int i = 1; i <= n; i++) {                              
			for(int j = 1;j<=m;j++) {
				update(i, j, f[i][j]);
			}
		} 
	}     
	int rsq(int i, int j) {                                     
    	long long sum = 0; 
		for (int x=i; x; x -= LSOne(x)) for(int y=j;y;y-=LSOne(y))  
			sum += ft[x][y];
    	return sum; 
	}

  	int rsq(int i1, int j1, int i2, int j2) { 
		return rsq(i2, j2) + rsq(i1-1, j1-1) - rsq(i1-1, j2) - rsq(i2, j1-1); 
	}  

};

class FenwickTreeXor {              
private: 
	std::vector<long long> ft; 
	int n;  
	std::vector<int> prev;  
public: 
	FenwickTreeXor(int _n) : n(_n) { 
		ft.assign(n+1, 0); 
		prev.assign(n+1, -1); 
	}   
	long long query(int j) {                                     // returns RSQ(1, j)
    	long long ans = 0LL; 
		for (; j; j -= LSOne(j)) ans ^= ft[j];
   		return ans; 
	}
  	long long query(int i, int j) { 
		  return query(j) ^ query(i-1); 
	}  

	/**
	Update i-th element to have v bit on
	To count number of bit on, use
	__builtin_popcount for int
	__builtin_popcountlong long for long long
	*/
	void update(int i, int v) {
	    int current_i = i;
	  	if (prev[i] == -1) for (; i <= n; i += LSOne(i)) ft[i] ^= (1LL << v); 
		else {
			int prv = prev[current_i];
			for (; i <= n; i += LSOne(i)) {
				ft[i] ^= (1LL << prv); // turn off prev-th bit
				ft[i] ^= (1LL << v); // turn on v-th bit
			}
		}
		prev[current_i] = v;
	}   
};