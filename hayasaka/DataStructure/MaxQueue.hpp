#include <bits/stdc++.h>
using namespace std;

template <class T>
class MaxQueue {
private:
	deque<pair<T, int>> q;
	int cnt_added = 0;
	int cnt_removed = 0;
public:
	T get() {
		return q.back().first;
	}

	void push(T new_element) {
		while (!q.empty() && q.front().first <= new_element)
			q.pop_front();
		q.push_front({new_element, cnt_added});
		cnt_added++;
	}

	void pop() {
		if (!q.empty() && q.back().second == cnt_removed) 
			q.pop_back();
		cnt_removed++;
	}
};