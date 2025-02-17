#include <vector>
#include <numeric>
/**
Check if a denomination is canonical (can be solved by greedy)
denom[i] > denom[i-1] > denom[i-2] ...
*/
class CoinChange {
private:
	std::vector<int> denom, greedy, mini;
	int denom_size = 0;
	void getGreedy(long long amount) {
		for(int i = 1; i <= denom_size; ++i) {
			greedy[i] = amount / denom[i];
			amount %= denom[i];
		}
	}
public:
	// denom is 1-based indexing
	CoinChange(vector<int>& _denom) : denom_size(_denom.size() - 1), denom(_denom) {}
	bool isCanonical() {
		int n = denom_size;
		greedy.assign(denom.size(), 0);
		for(int i = 2; i <= n; ++i) {
			for(int j = i; j <= n; ++j) {
				getGreedy(denom[i-1]-1);
				if (greedy[i] == 0) continue;
				mini.assign(denom.size(), 0);
				for(int k = 0; k < j; ++k) mini[k] = greedy[k];
				mini[j] = greedy[j] + 1;
				long long w = 0LL;
				for(int k = 1; k <= n; ++k) w += mini[k] * denom[k];
				getGreedy(w);
				int ms = std::accumulate(mini.begin(),mini.end(), 0);
				int mg = std::accumulate(greedy.begin(),greedy.end(), 0);
				if (ms < mg) return false;
			}
		}
		return true;
	}
};
