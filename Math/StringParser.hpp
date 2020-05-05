#include <string>
/**
Python Eval in C++
*/
class Parser {
private:
	std::string input;
	int stringidx = 0;
	inline char peek() {
		return input[stringidx];
	}

	inline char get() {
		return input[stringidx++];
	}

	inline int expression();

	inline int number() {
		int result = get() - '0';
		while (peek() >= '0' && peek() <= '9') {
			result = 10*result + get() - '0';
		}
		return result;
	}

	inline int factor() {
		if (isalpha(peek())) {
			return g[get()];
		} else if (peek() == '(') {
			get(); // '('
			int result = expression();
			get(); // ')'
			return result;
		}
		return 0; // error
	}

	inline int term() {
		int result = factor();
		while (stringidx < input.length() && (peek() == '*' || peek() == '/'))
			if (get() == '*')
				result *= factor();
		return result;
	}

	inline int expression() {
		int result = term();
		while (stringidx < input.length() && (peek() == '+' || peek() == '-'))
			if (get() == '+')
				result += term();
			else
				result -= term();
		return result;
	}
public:
	void compute(std::string& s) {
		input = s;
		stringidx = 0;
		return expression();
	}
}

