

/**
SUFFIX Array
*/

class SuffixArray {
private:
	static const int MAX_N = 100010;
	string T, P;
	int n;	// the length of input string
	int RA[MAX_N] {0}, tempRA[MAX_N] {0};	// rank array and temporary rank array
	int SA[MAX_N] {0}, tempSA[MAX_N];	// suffix array and temporary suffix array
	int c[MAX_N] {0};	// for counting/radix sort
	int Phi[MAX_N] {0};	// for computing longest common prefix
	int PLCP[MAX_N] {0};
	int LCP[MAX_N] {0};	// LCP[i] stores the LCP between previous suffix T+SA[i-1]
	// and current suffix T+SA[i]

	void countingSort(int k) {	
		int i, sum, maxi = max(300, n);	 
		memset(c, 0, sizeof c);
		for (i = 0; i < n; i++)	 c[i+k < n ? RA[i+k] : 0]++;
		for (i = sum = 0; i < maxi; i++) {
			int t = c[i]; c[i] = sum; sum += t;
		}
		for (i = 0; i < n; i++) tempSA[c[SA[i]+k < n ? RA[SA[i]+k] : 0]++] = SA[i];
		for (i = 0; i < n; i++)	SA[i] = tempSA[i];
	}

	void constructSA() {	 // this version can go up to 100000 characters
		int i, k, r;
		for (i = 0; i < n; i++) RA[i] = T[i];	 
		for (i = 0; i < n; i++) SA[i] = i;	
		for (k = 1; k < n; k <<= 1) {	
			countingSort(k);
			countingSort(0);	
			tempRA[SA[0]] = r = 0;
			for (i = 1; i < n; i++)	tempRA[SA[i]] = 
				(RA[SA[i]] == RA[SA[i-1]] && RA[SA[i]+k] == RA[SA[i-1]+k]) ? r : ++r;
			for (i = 0; i < n; i++)	RA[i] = tempRA[i];
			if (RA[SA[n-1]] == n-1) break;	 // nice optimization trick
		} 
	}

	void computeLCP() {
		int i, L;
		Phi[SA[0]] = -1;	
		for (i = 1; i < n; i++) Phi[SA[i]] = SA[i-1];	
		for (i = L = 0; i < n; i++) {	
			if (Phi[i] == -1) { 
				PLCP[i] = 0; 
				continue; 
			}
			while (T[i+L] == T[Phi[i]+L]) ++L;
			PLCP[i] = L;
			L = max(L-1, 0);
		}
		for (i = 0; i < n; i++)	LCP[i] = PLCP[SA[i]];	
	}
public:
	SuffixArray(string _T) : T(_T) {
		n = T.size();
		constructSA();
	};

	void debugSA() {
		cout <<"SA" <<'\n';
		for(int i = 0 ; i < n ; ++i) {
			cout << i + 1 << "	-> " << SA[i] <<'\n';
			for(int j = SA[i]; j < n; ++j) cout << T[j];
			cout <<'\n';
		}
		cout <<"\n";
	}
};




class SuffixArrayAlternative {
private:
	vector<int> SA, c, k_array, lcp;
	int n;
	string s;
	string _p;

	void sort_cyclic_shifts() {
		const int alphabet = 256;
		vector<int> cnt(max(alphabet, n), 0);
		SA.resize(n);
		c.resize(n);
		for (int i = 0; i < n; i++)
			cnt[s[i]]++;
		for (int i = 1; i < alphabet; i++)
			cnt[i] += cnt[i-1];
		for (int i = 0; i < n; i++)
			SA[--cnt[s[i]]] = i;
		c[SA[0]] = 0;
		int classes = 1;
		for (int i = 1; i < n; i++) {
			if (s[SA[i]] != s[SA[i-1]])
				classes++;
			c[SA[i]] = classes - 1;
		}

		vector<int> pn(n), cn(n);
		for (int h = 0; (1 << h) < n; ++h) {
			for (int i = 0; i < n; i++) {
				pn[i] = SA[i] - (1 << h);
				if (pn[i] < 0)
					pn[i] += n;
			}
			fill(cnt.begin(), cnt.begin() + classes, 0);
			for (int i = 0; i < n; i++)
				cnt[c[pn[i]]]++;
			for (int i = 1; i < classes; i++)
				cnt[i] += cnt[i-1];
			for (int i = n-1; i >= 0; i--)
				SA[--cnt[c[pn[i]]]] = pn[i];
			cn[SA[0]] = 0;
			classes = 1;
			for (int i = 1; i < n; i++) {
				pair<int, int> cur = {c[SA[i]], c[(SA[i] + (1 << h)) % n]};
				pair<int, int> prev = {c[SA[i-1]], c[(SA[i-1] + (1 << h)) % n]};
				if (cur != prev)
					++classes;
				cn[SA[i]] = classes - 1;
			}
			c.swap(cn);
		}
		SA.erase(SA.begin());
	}

	int compare(int SA_index) {
		int t_i = SA[SA_index];
		for (int i = 0; i < _p.length(); ++i) {
			if (t_i == n) return 0;
			if (_p[i] != s[t_i]) return _p[i] < s[t_i];
			++t_i;
		}
		return 2;
	}

public:
	SuffixArrayAlternative(string& _s, bool append=true) : s(_s), n(_s.length() + 1) {
		if (append) s += "$";
		sort_cyclic_shifts();
	}

	vector<int> find_pattern(const string& p) {
		_p = p;
		int lo = -1, hi = SA.size() - 1;
		while(lo + 1 < hi) {
			int mid = (lo + hi) >> 1;
			if (compare(mid) != 0) {
				hi = mid;
			} else {
				lo = mid;
			}
		}
		if (compare(hi) != 2) return {};
		int lwb = hi;
		lo = 0, hi = SA.size();
		while(lo + 1 < hi) {
			int mid = (lo + hi) >> 1;
			if (compare(mid) != 1) {
				lo = mid;
			} else {
				hi = mid;
			}
		}
		int upb = lo;
		vector<int> ans;
		for (int i = lwb; i <= upb; ++i) {
			ans.push_back(SA[i]);
		}
		sort(ans.begin(), ans.end());
		return ans;

	}

	void lcp_construction() {
		lcp.assign(n-1, 0);
		vector<int> rank(n, 0);
		for (int i = 0; i < n; i++)
			rank[SA[i]] = i;

		int k = 0;
		for (int i = 0; i < n; i++) {
			if (rank[i] == n - 1) {
				k = 0;
				continue;
			}
			int j = SA[rank[i] + 1];
			while (i + k < n && j + k < n && s[i+k] == s[j+k])
				k++;
			lcp[rank[i]] = k;
			if (k)
				k--;
		}
	}

	long long number_of_unique_substrings() {
		lcp_construction();
		return ((long long) (n - 1) * (n - 1) + (n - 1)) / 2 - accumulate(lcp.begin(), lcp.end(), 0LL);
	}
	/**
	Comparing two substrings of a string
	We want to be able to compare two substrings of the same length of a given 
	string s in O(1) time, i.e. checking if the first substring is smaller than the second one.
	Let's compare two substrings of length l with the starting indices i and j. We 
	find the largest length of a block that is placed inside a substring of this length: t
	he greatest k such that 2k≤l. Then comparing the two substrings can be replaced by comparing 
	two overlapping blocks of length 2k: first you need to compare the two blocks starting at 
	i and j, and if these are equal then compare the two blocks ending in positions i+l−1 and j+l−1:
	*/
	// int compare(int i, int j, int l) {
	// 	if (k_array.empty()) {
	// 		k_array.resize(n + 1);
	// 		int d = 0;
	// 		for(int i = 1; i <= n; ++i) {
	// 			while(1 << (d + 1) <= i) ++d;
	// 			k_array[i] = d;
	// 		}
	// 	}
	// 	int k = k_array[l];
	// 	pair<int, int> a = { c[k][i], c[k][(i+l-(1 << k))%n] };
	// 	pair<int, int> b = { c[k][j], c[k][(j+l-(1 << k))%n] };
	// 	return a == b ? 0 : a < b ? -1 : 1;
	// }
};