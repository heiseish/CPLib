#include <bits/stdc++.h>
using namespace std;

template <class T>
class MinQueue {
private:
	deque<pair<T, int>> q;
	int cnt_added = 0;
	int cnt_removed = 0;
public:
	T get() {
		return q.front().first;
	}

	void push(T new_element) {
		while (!q.empty() && q.back().first >= new_element)
			q.pop_back();
		q.push_back({new_element, cnt_added});
		cnt_added++;
	}

	void pop() {
		if (!q.empty() && q.front().second == cnt_removed) 
			q.pop_front();
		cnt_removed++;
	}
};
