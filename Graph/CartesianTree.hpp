#include <vector>
#include <stack>


/**
* Build a cartesian tree from an array in O(N)
* This tree can be used by LCA by the Farach-Colton and Bender to solve RMQ in O(1)
* with O(N) preprocessing
*/
class CartesianTree {
private:
	int n;
	std::vector<int> parent;
	std::vector<int> A;
	std::stack<int> s;
	void build() {
		for (int i = 0; i < n; ++i ) {
		    int last = -1;
		    while (!s.empty() && A[s.top()] >= A[i]) {
		        last = s.top();
		        s.pop();
		    }
		    if (!s.empty())
		        parent[i] = s.top();
		    if (last >= 0)
		        parent[last] = i;
		    s.push(i);
		}
	}
	
public:
	CartesianTree(const std::vector<int>& a) : A(a), n(a.size()) {
		parent.assign(n, -1);
		build();
	}
};