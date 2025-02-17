fn lcs(s: &[char], t: &[char]) -> usize {
    let n = s.len();
    let m = t.len();

    let mut prev = vec![0; m + 1];
    let mut cur = vec![0; m + 1];
    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1] == t[j - 1] {
                cur[j] = 1 + prev[j - 1];
            } else {
                cur[j] = cur[j - 1].max(prev[j])
            }
        }
        prev = cur.clone();
    }
    cur[m]
}

fn lcs_recover(s: &[char], t: &[char]) -> String {
    let m = s.len();
    let n = t.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut index = dp[m][n];
    let mut lcs = vec!['a'; index];

    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if s[i - 1] == t[j - 1] {
            lcs[index - 1] = s[i - 1];
            i -= 1;
            j -= 1;
            index -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.into_iter().collect::<String>()
}
