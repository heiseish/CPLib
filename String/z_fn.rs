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
