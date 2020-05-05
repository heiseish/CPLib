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





int main() {
	return 0;
}