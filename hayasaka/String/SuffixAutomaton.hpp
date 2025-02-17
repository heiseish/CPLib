#include <bits/stdc++.h>
using namespace std;

class SuffixAutomaton {
private:
	static const int MAXLEN = 100000;
	vector<state> st;
	int sz, last;
	struct state {
		int len, link;
		map<char, int> next;
	};

	void sa_init() {
		st.resize(MAXLEN * 2);
		st[0].len = 0;
		st[0].link = -1;
		sz++;
		last = 0;
	}

	void sa_extend(char c) {
		int cur = sz++;
		st[cur].len = st[last].len + 1;
		int p = last;
		while (p != -1 && !st[p].next.count(c)) {
			st[p].next[c] = cur;
			p = st[p].link;
		}
		if (p == -1) {
			st[cur].link = 0;
		} else {
			int q = st[p].next[c];
			if (st[p].len + 1 == st[q].len) {
				st[cur].link = q;
			} else {
				int clone = sz++;
				st[clone].len = st[p].len + 1;
				st[clone].next = st[q].next;
				st[clone].link = st[q].link;
				while (p != -1 && st[p].next[c] == q) {
					st[p].next[c] = clone;
					p = st[p].link;
				}
				st[q].link = st[cur].link = clone;
			}
		}
		last = cur;
	}

public:
	SuffixAutomaton(const string& t) {
		sa_init();
		for (auto ch : t) sa_extend(ch);
	}
}
int main() {
	return 0;
}