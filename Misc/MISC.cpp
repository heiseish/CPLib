#include <bits/stdc++.h>
using namespace std;

vector<int> min_sliding_window(const vector<int>& A, const int& k) {
	int n = A.size();
	deque<pair<int, int> > window;
	vector<int> ans;
	for(int i = 0; i < n; ++i) {
		while(!window.empty() && window.back().first >= A[i])
			window.pop_back();

		window.push_back(pair<int, int>(A[i], i));
		while(window.front().second <= i - K)
			window.pop_front();

		if (i + 1 >= K) 
			ans.push_back(window.front().first);
	}
	return ans;
}

vector<int> max_sliding_window(const vector<int>& A, const int& k) {
	int n = A.size();
	deque<pair<int, int> > window;
	vector<int> ans;
	for(int i = 0; i < n; ++i) {
		while(!window.empty() && window.front().first <= A[i])
			window.pop_front();

		window.push_front(pair<int, int>(A[i], i));
		while(window.back().second <= i - K)
			window.pop_back();

		if (i + 1 >= K) 
			ans.push_back(window.back().first);
	}
	return ans;
}