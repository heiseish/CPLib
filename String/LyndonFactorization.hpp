
/**
Lyndon factorization
First let us define the notion of the Lyndon factorization.

A string is called simple (or a Lyndon word), if it is strictly smaller than any of its own 
nontrivial suffixes. Examples of simple strings are: a, b, ab, aab, abb, ababb, abcd. 
It can be shown that a string is simple, if and only if it is strictly smaller than all its 
nontrivial cyclic shifts.

Next, let there be a given string s. The Lyndon factorization of the string s is a factorization 
s=w1w2…wk, where all strings wi are simple, and they are in non-increasing order w1≥w2≥⋯≥wk.

It can be shown, that for any string such a factorization exists and that it is unique.

Duval algorithm
The Duval algorithm constructs the Lyndon factorization in O(n) time using O(1) additional memory.
*/
vector<string> duval(string const& s) {
    int n = s.size();
    int i = 0;
    vector<string> factorization;
    while (i < n) {
        int j = i + 1, k = i;
        while (j < n && s[k] <= s[j]) {
            if (s[k] < s[j])
                k = i;
            else
                k++;
            j++;
        }
        while (i <= k) {
            factorization.push_back(s.substr(i, j - k));
            i += j - k;
        }
    }
    return factorization;
}
