fn buildz(n: usize, s: &[char], z: &mut [usize]) {
    // s is 0-indexed char array
    let mut l = 0;
    let mut r = 0;
    for i in 0..n {
        z[i] = 0;
    }
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
}

// n - size of string. string starts from 0th
pub fn build_prefix(n: usize, s: &[char], pi: &mut [char]) {
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
}
