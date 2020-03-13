#include <bits/stdc++.h>
using namespace std;

template <class T>
class MaxStack {
private:
	stack<pair<T, T> > st;
public:
	void push(T new_element) {
		T new_max = st.empty() ? new_element : max<T>(new_element, st.top().second);
		st.push({ new_element, new_max });
	}
	void pop() {
		T removed_element = st.top().first;
		st.pop();
	}

	T get() {
		return st.top().second;
	}
};
