
/**
Manaches's algorithm
Given string s with length n. Find all the pairs (i,j) such that substring s[i…j] is a palindrome. 
String t is a palindrome when t=trev (trev is a reversed string for t).

But the information about the palindromes can be kept in a more compact way: for each position i=0…n−1 we'll find the values d1[i] and d2[i], 
denoting the number of palindromes accordingly with odd and even lengths with centers in the position i.
*/
long long number_of_sub_palidromes(string& s) {
	vector<int> d1(n);
	for (int i = 0, l = 0, r = -1; i < n; i++) {
		int k = (i > r) ? 1 : min(d1[l + r - i], r - i + 1);
		while (0 <= i - k && i + k < n && s[i - k] == s[i + k]) {
			k++;
		}
		d1[i] = k--;
		if (i + k > r) {
			l = i - k;
			r = i + k;
		}
	}

	vector<int> d2(n);
	for (int i = 0, l = 0, r = -1; i < n; i++) {
		int k = (i > r) ? 0 : min(d2[l + r - i + 1], r - i + 1);
		while (0 <= i - k - 1 && i + k < n && s[i - k - 1] == s[i + k]) {
			k++;
		}
		d2[i] = k--;
		if (i + k > r) {
			l = i - k - 1;
			r = i + k ;
		}
	}
	return accumulate(d1.begin(), d1.end(), 0LL) + accumulate(d2.begin(), d2.end(), 0LL);
}
