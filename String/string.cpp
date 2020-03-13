/** 
Wherever someone is still thinking of you, that place is your home.        
*/
#include <bits/stdc++.h>
using namespace std;

// Polynomial rolling hashing function
long long compute_hash(string const& s) {
    const int p = 31; // close prime number > number of char (26 in this case)
    const int m = 1e9 + 9; // large prime
    long long hash_value = 0;
    long long p_pow = 1;
    for (char c : s) {
        hash_value = (hash_value + (c - 'a' + 1) * p_pow) % m;
        p_pow = (p_pow * p) % m;
    }
    return hash_value;
}

/**
Search for duplicate strings in an array of strings
Problem: Given a list of n strings si, each no longer than m characters, find all the duplicate strings and divide them into groups.

From the obvious algorithm involving sorting the strings, we would get a time complexity of O(nmlogn) where the sorting requires O(nlogn) comparisons and each comparison take O(m) time. However by using hashes, we reduce the comparison time to O(1), giving us an algorithm that runs in O(nm+nlogn) time.
*/
vector<vector<int> > group_identical_strings(vector<string> const& s) {
    int n = s.size();
    vector<pair<long long, int> > hashes(n);
    for (int i = 0; i < n; ++i)
        hashes[i] = { compute_hash(s[i]), i };

    sort(hashes.begin(), hashes.end());

    vector<vector<int>> groups;
    for (int i = 0; i < n; i++) {
        if (i == 0 || hashes[i].first != hashes[i-1].first)
            groups.emplace_back();
        groups.back().push_back(hashes[i].second);
    }
    return groups;
}

/**
Determine the number of different substrings in a string
O(m^2) 
*/
int count_unique_substrings(string const& s) {
    int n = s.size();

    const int p = 31;
    const int m = 1e9 + 9;
    vector<long long> p_pow(n);
    p_pow[0] = 1;
    for (int i = 1; i < n; i++)
        p_pow[i] = (p_pow[i-1] * p) % m;

    vector<long long> h(n + 1, 0);
    for (int i = 0; i < n; i++)
        h[i+1] = (h[i] + (s[i] - 'a' + 1) * p_pow[i]) % m;

    int cnt = 0;
    for (int l = 1; l <= n; l++) {
        set<long long> hs;
        for (int i = 0; i <= n - l; i++) {
            long long cur_h = (h[i + l] + m - h[i]) % m;
            cur_h = (cur_h * p_pow[n-i-1]) % m;
            hs.insert(cur_h);
        }
        cnt += hs.size();
    }
    return cnt;
}


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
/**
Find pattern in text in O(n + m) using hash
*/
vector<int> rabin_karp(string const& s, string const& t) {
    const int p = 31; 
    const int m = 1e9 + 9;
    int S = s.size(), T = t.size();

    vector<long long> p_pow(max(S, T)); 
    p_pow[0] = 1; 
    for (int i = 1; i < (int)p_pow.size(); i++) 
        p_pow[i] = (p_pow[i-1] * p) % m;

    vector<long long> h(T + 1, 0); 
    for (int i = 0; i < T; i++)
        h[i+1] = (h[i] + (t[i] - 'a' + 1) * p_pow[i]) % m; 
    long long h_s = 0; 
    for (int i = 0; i < S; i++) 
        h_s = (h_s + (s[i] - 'a' + 1) * p_pow[i]) % m; 

    vector<int> occurences;
    for (int i = 0; i + S - 1 < T; i++) { 
        long long cur_h = (h[i+S] + m - h[i]) % m; 
        if (cur_h == h_s * p_pow[i] % m)
            occurences.push_back(i);
    }
    return occurences;
}

inline vector<int> prefix_function(string const& s) {
    int n = (int)s.length();
    vector<int> pi(n);
    for (int i = 1; i < n; i++) {
        int j = pi[i-1];
        while (j > 0 && s[i] != s[j])
            j = pi[j-1];
        if (s[i] == s[j])
            j++;
        pi[i] = j;
    }
    return pi;
}

/**
String matching in O(m + n)
*/
inline vector<int> KMP(string const&p, string const& t) {
	int n = t.length(), m = p.length();
	vector<int> pi(max(n, m) + 1);
	int i = 0, j = -1; pi[0] = -1; 
	while (i < m) {
		while (j >= 0 && p[i] != p[j]) j = pi[j]; 
		i++; j++; 
		pi[i] = j;
	} 
	vector<int> ans;
	i = 0, j = 0;
	while (i < n) {
		while (j >= 0 && t[i] != p[j]) j = pi[j]; 
		i++; j++; 
		if (j == m) { 
			ans.push_back(i-j);
			j = pi[j]; 
		} 
	} 
	return ans;
}


/**
Counting the number of occurrences of each prefix
Here we discuss two problems at once. Given a string s of length n. In the first variation of 
the problem we want to count the number of appearances of each prefix s[0…i] in the same string. 
In the second variation of the problem another string t is given and we want to count the number 
of appearances of each prefix s[0…i] in t.

First we solve the first problem. Consider the value of the prefix function π[i] at a position i. 
By definition it means that in position i the prefix of length π[i] of the string s appears and 
ends at position i, and there doesn't exists a longer prefix that follows this definition. 
At the same time shorter prefixes can end at this position. It is not difficult to see, that we 
have the same question that we already answered when we computed the prefix function itself: 
Given a prefix of length j that is a suffix ending at position i, what is the next smaller 
prefix <j that is also a suffix ending at position i. Thus at the position i ends the prefix of 
length π[i], the prefix of length π[π[i]−1], the prefix π[π[π[i]−1]−1], and so on, until the 
index becomes zero. Thus we can compute the answer in the following way.
*/
vector<int> occurence_of_prefix(string& s) {
	int n = (int)s.length();
	vector<int> pi = prefix_function(s);
	vector<int> ans(n + 1);
	for (int i = 0; i < n; i++)
		ans[pi[i]]++;
	for (int i = n-1; i > 0; i--)
		ans[pi[i-1]] += ans[i];
	for (int i = 0; i <= n; i++)
		ans[i]++;
	return ans;
}

/**
Compressing a string
Given a string s of length n. We want to find the shortest 
"compressed" representation of the string, i.e. we want to 
find a string t of smallest length such that s can be represented as a concatenation of 
one or more copies of t.

It is clear, that we only need to find the length of t. Knowing the length, the answer 
to the problem will be the prefix of s with this length.

Let us compute the prefix function for s. Using the last value of it we define the 
value k=n−π[n−1]. We will show, that if k divides n, then k will be the answer, otherwise 
there doesn't exists an effective compression and the answer is n.
*/
inline string compress(string const& s) {
    int n = (int)s.length();
    vector<int> pi(n);
    for (int i = 1; i < n; i++) {
        int j = pi[i-1];
        while (j > 0 && s[i] != s[j])
            j = pi[j-1];
        if (s[i] == s[j])
            j++;
        pi[i] = j;
    }
	if ( n % (n - pi[n-1]) != 0) return s;
	else return s.substr(0, n - pi[n-1]);
}


void compute_automaton(string s, vector<vector<int>>& aut) {
    s += '#';
    int n = s.size();
    vector<int> pi = prefix_function(s);
    aut.assign(n, vector<int>(26));
    for (int i = 0; i < n; i++) {
        for (int c = 0; c < 26; c++) {
            if (i > 0 && 'a' + c != s[i])
                aut[i][c] = aut[pi[i-1]][c];
            else
                aut[i][c] = i + ('a' + c == s[i]);
        }
    }
}

/**
Z-function
Suppose we are given a string s of length n. 
The Z-function for this string is an array of length n where 
the i-th element is equal to the greatest number of characters 
starting from the position i that coincide with the first 
characters of s.

In other words, z[i] is the length of the longest common 
prefix between s and the suffix of s starting at i.
*/
inline vector<int> z_function(const string& s) {
    int n = (int) s.length();
    vector<int> z(n);
    for (int i = 1, l = 0, r = 0; i < n; ++i) {
        if (i <= r)
            z[i] = min (r - i + 1, z[i - l]);
        while (i + z[i] < n && s[z[i]] == s[i + z[i]])
            ++z[i];
        if (i + z[i] - 1 > r)
            l = i, r = i + z[i] - 1;
    }
    return z;
}

/**
With z-function
*/
inline vector<int> find_pattern_in_string(const string& t, const string & p) {
	string s = p + '#' + t;
	vector<int> z = z_function(s);
	vector<int> ans;
	for(int i = p.length() + 1; i < s.length(); ++i) {
		if (z[i] == p.length()) {
			ans.push_back(i - p.length() - 1);
		}
	}
	return ans;
}

class SuffixTrie {
private:
	const static int K = 26; // number of character
	struct Vertex {
		vector<int> next;
		bool leaf = false;
		Vertex() {
			next.assign(K, -1);
		}
	};
	vector<Vertex> trie;
public:
	SuffixTrie() { trie.push_back({}); }
	void add_string(string const& s) {
    	int v = 0;
		for (char ch : s) {
			int c = ch - 'a';
			if (trie[v].next[c] == -1) {
				trie[v].next[c] = trie.size();
				trie.emplace_back();
			}
			v = trie[v].next[c];
		}
		trie[v].leaf = true;
	}

};

class AhoCorasick {
private:
	static const int K = 26;
	int num_pattern = 0;
	struct Vertex {
		vector<int> next, go, pattern_index;
		bool leaf = false;
		int p = -1;
		char pch;
		int link = -1;
		int exit_link = -1;
		int pattern_length = -1;
		Vertex(int p=-1, char ch='$') : p(p), pch(ch) {
			next.assign(K, -1);
			go.assign(K, -1);
		}
	};	

	int toInt(char ch) {
		return ch - 'A';
	}
public:
	vector<Vertex> t;
	AhoCorasick() { t.push_back({}); }
	void add_string(string const& s, int idx=0) {
		
		int v = 0;
		for (char ch : s) {
			int c = toInt(ch);
			if (t[v].next[c] == -1) {
				t[v].next[c] = t.size();
				t.emplace_back(v, ch);
			}
			t[t[v].next[c]].p = v;
			t[t[v].next[c]].pch = ch;
			v = t[v].next[c];
		}
		t[v].leaf = true;
		t[v].pattern_index.push_back(idx);
		t[v].pattern_length = s.length();
		num_pattern = max(num_pattern, idx);
	}

	int get_link(int v) {
		if (t[v].link == -1) {
			if (v == 0 || t[v].p == 0)
				t[v].link = 0;
			else
				t[v].link = go(get_link(t[v].p), t[v].pch);
		}
		return t[v].link;
	}

	int go(int v, char ch) {
		int c = toInt(ch);
		if (t[v].go[c] == -1) {
			if (t[v].next[c] != -1)
				t[v].go[c] = t[v].next[c];
			else
				t[v].go[c] = v == 0 ? 0 : go(get_link(v), ch);
		}
		return t[v].go[c];
	} 
	
	int get_exit_link(int v) {
		if (t[v].exit_link == -1) {
			int k = get_link(v);
			while(k != 0 && !t[k].leaf) k = get_link(k);
			t[v].exit_link = k;
		}
		return t[v].exit_link;
	}

	vector<vector<int> > search_for_occurence_in_text(const string& text) {
		vector<vector<int> > ans(num_pattern, vector<int>());
		int idx = 1;
		int v = 0;
		for (auto &ch : text) {
			v = go(v, ch);
			int k = v;
			do {
				if (t[k].leaf) {
					for (auto &s : t[k].pattern_index) {
						ans[s].push_back(idx - t[k].pattern_length);
					}
				}
				k = get_exit_link(k);
			} while(k != 0);
			idx++;
		}
		return ans;
	}
};

/**
Lyndon factorization
First let us define the notion of the Lyndon factorization.

A string is called simple (or a Lyndon word), if it is strictly smaller than any of its own 
nontrivial suffixes. Examples of simple strings are: a, b, ab, aab, abb, ababb, abcd. 
It can be shown that a string is simple, if and only if it is strictly smaller than all its 
nontrivial cyclic shifts.

Next, let there be a given string s. The Lyndon factorization of the string s is a factorization 
s=w1w2…wk, where all strings wi are simple, and they are in non-increasing order w1≥w2≥⋯≥wk.

It can be shown, that for any string such a factorization exists and that it is unique.

Duval algorithm
The Duval algorithm constructs the Lyndon factorization in O(n) time using O(1) additional memory.
*/
vector<string> duval(string const& s) {
    int n = s.size();
    int i = 0;
    vector<string> factorization;
    while (i < n) {
        int j = i + 1, k = i;
        while (j < n && s[k] <= s[j]) {
            if (s[k] < s[j])
                k = i;
            else
                k++;
            j++;
        }
        while (i <= k) {
            factorization.push_back(s.substr(i, j - k));
            i += j - k;
        }
    }
    return factorization;
}


string min_cyclic_string(string s) {
    s += s;
    int n = s.size();
    int i = 0, ans = 0;
    while (i < n / 2) {
        ans = i;
        int j = i + 1, k = i;
        while (j < n && s[k] <= s[j]) {
            if (s[k] < s[j])
                k = i;
            else
                k++;
            j++;
        }
        while (i <= k)
            i += j - k;
    }
    return s.substr(ans, n / 2);
}


/**
Manaches's algorithm
Given string s with length n. Find all the pairs (i,j) such that substring s[i…j] is a palindrome. 
String t is a palindrome when t=trev (trev is a reversed string for t).

But the information about the palindromes can be kept in a more compact way: for each position i=0…n−1 we'll find the values d1[i] and d2[i], 
denoting the number of palindromes accordingly with odd and even lengths with centers in the position i.
*/
long long number_of_sub_palidromes(string& s) {
	vector<int> d1(n);
	for (int i = 0, l = 0, r = -1; i < n; i++) {
		int k = (i > r) ? 1 : min(d1[l + r - i], r - i + 1);
		while (0 <= i - k && i + k < n && s[i - k] == s[i + k]) {
			k++;
		}
		d1[i] = k--;
		if (i + k > r) {
			l = i - k;
			r = i + k;
		}
	}

	vector<int> d2(n);
	for (int i = 0, l = 0, r = -1; i < n; i++) {
		int k = (i > r) ? 0 : min(d2[l + r - i + 1], r - i + 1);
		while (0 <= i - k - 1 && i + k < n && s[i - k - 1] == s[i + k]) {
			k++;
		}
		d2[i] = k--;
		if (i + k > r) {
			l = i - k - 1;
			r = i + k ;
		}
	}
	return accumulate(d1.begin(), d1.end(), 0LL) + accumulate(d2.begin(), d2.end(), 0LL);
}


/**
Main-Lorentz algorithm
The idea behind the Main-Lorentz algorithm is divide-and-conquer.

It splits the initial string into halves, and computes the number of repetitions that 
lie completely in each halve by two recursive calls. Then comes the difficult part. 
The algorithm finds all repetitions starting in the first half and ending in the second 
half (which we will call crossing repetitions). This is the essential part of the Main-Lorentz 
algorithm, and we will discuss it in detail here.

The complexity of divide-and-conquer algorithms is well researched. 
The master theorem says, that we will end up with an O(nlogn) algorithm, 
if we can compute the crossing repetitions in O(n) time.
*/
int get_z(vector<int> const& z, int i) {
    if (0 <= i && i < (int)z.size())
        return z[i];
    else
        return 0;
}

vector<pair<int, int> > repetitions;

void convert_to_repetitions(int shift, bool left, int cntr, int l, int k1, int k2) {
    for (int l1 = max(1, l - k2); l1 <= min(l, k1); l1++) {
        if (left && l1 == l) break;
        int l2 = l - l1;
        int pos = shift + (left ? cntr - l1 : cntr - l - l1 + 1);
        repetitions.emplace_back(pos, pos + 2*l - 1);
    }
}

void find_repetitions(string s, int shift = 0) {
    int n = s.size();
    if (n == 1)
        return;

    int nu = n / 2;
    int nv = n - nu;
    string u = s.substr(0, nu);
    string v = s.substr(nu);
    string ru(u.rbegin(), u.rend());
    string rv(v.rbegin(), v.rend());

    find_repetitions(u, shift);
    find_repetitions(v, shift + nu);

    vector<int> z1 = z_function(ru);
    vector<int> z2 = z_function(v + '#' + u);
    vector<int> z3 = z_function(ru + '#' + rv);
    vector<int> z4 = z_function(v);

    for (int cntr = 0; cntr < n; cntr++) {
        int l, k1, k2;
        if (cntr < nu) {
            l = nu - cntr;
            k1 = get_z(z1, nu - cntr);
            k2 = get_z(z2, nv + 1 + cntr);
        } else {
            l = cntr - nu + 1;
            k1 = get_z(z3, nu + 1 + nv - 1 - (cntr - nu));
            k2 = get_z(z4, (cntr - nu) + 1);
        }
        if (k1 + k2 >= l)
            convert_to_repetitions(shift, cntr < nu, cntr, l, k1, k2);
    }
}

int main() {
	return 0;
}