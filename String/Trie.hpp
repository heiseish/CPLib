#include <vector>
#include <string>
class Trie {
private:
	int ALPHA = 26;
	int root = 0;
public:
	std::vector<std::vector<int> > trie;
	std::vector<int> occur;
	std::vector<int> par;
	Trie(int dict=26) : ALPHA(dict) {
		trie.emplace_back(ALPHA, -1);
		par.emplace_back(-1);
		occur.emplace_back(0);
	}

	void addWords(std::string &s) {
		int t = 0;
		for (char c : s) {
			int d = (int) (c - 'a');
			if (trie[t][d] == -1) {
				trie[t][d] = (int) trie.size();
				trie.emplace_back(ALPHA, -1);
				occur.push_back(0);
				par.push_back(t);
			}
			t = trie[t][d];
			occur[t]++;
		}
	}

	int checkWords(std::string &s) {
		int prev = root;
		int ct = 0;
		int nxt = s[ct] - 'a';
		while(ct < s.length() - 1) {
			if (trie[prev][nxt] == -1) return 0;
			prev = trie[prev][nxt];
			ct++;
			nxt = s[ct] - 'a';
		}
		return trie[prev][nxt];
	}		
};