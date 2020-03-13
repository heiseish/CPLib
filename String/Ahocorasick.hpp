#include <bits/stdc++.h>
using namespace std;

class AhoCorasick {
private:
	static const int K = 26; // number of character
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
		if (t[v].exit_link != -1) return t[v].exit_link;
		int k = get_link(v);
		if (k == 0 || t[k].leaf) return t[v].exit_link = k;
		return t[v].exit_link = get_exit_link(k);
	}

	vector<vector<int> > search_for_occurence_in_text(const string& text) {
		vector<vector<int> > ans(num_pattern, vector<int>());
		int idx = 1;
		int v = 0;
		for (auto &ch : text) {
			v = go(v, ch);
			int k = v;
			while(k != 0) {
				if (t[k].leaf) 
					for (auto &s : t[k].pattern_index) 
						ans[s].push_back(idx - t[k].pattern_length);
				k = get_exit_link(k);
			} 
			idx++;
		}
		return ans;
	}
};

/**
Finding the lexicographical smallest string of a given length that doesn't match any given strings
A set of strings and a length L is given. We have to find a string of length L, 
which does not contain any of the string, and derive the lexicographical smallest of such strings.

We can construct the automaton for the set of strings. Let's remember, that the vertices 
from which we can reach a leaf vertex are the states, at which we have a match with a string 
from the set. Since in this task we have to avoid matches, we are not allowed to enter such states. 
On the other hand we can enter all other vertices. Thus we delete all "bad" vertices 
from the machine, and in the remaining graph of the automaton we find the lexicographical 
smallest path of length L. 
This task can be solved in O(L) for example by depth first search.
*/

/**
Finding the shortest string containing all given strings
Here we use the same ideas. For each vertex we store a mask that denotes the strings which match 
at this state. Then the problem can be reformulated as follows: initially being in the 
state (v=root, mask=0), we want to reach the state (v, mask=2n−1), where n is the number of 
strings in the set. When we transition from one state to another using a letter, we update the 
mask accordingly. 
By running a breath first search we can find a path to the state (v, mask=2n−1) with the smallest length.
*/

/**
Finding the lexicographical smallest string of length L containing k strings
As in the previous problem, we calculate for each vertex the number of matches that correspond to it 
(that is the number of marked vertices reachable using suffix links). We reformulate the problem: 
the current state is determined by a triple of numbers (v, len, cnt), and we want to reach from the 
state (root, 0, 0) the state (v, L, k), where v can be any vertex. Thus we can find such a path using 
depth first search (and if the search looks at the edges in their natural order, then the found path 
will automatically be the lexicographical smallest).
*
int main() {
	return 0;
}