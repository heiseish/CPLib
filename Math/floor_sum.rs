// https://atcoder.jp/contests/practice2/tasks/practice2_c
//  O(log a+log m)
fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }
    let y_max = a * (n - 1) + b;
    if y_max < m {
        return ans;
    }
    let y_max_div = y_max / m;
    let y_max_mod = y_max % m;
    ans += y_max_div + floor_sum(y_max_div, a, m, y_max_mod);
    ans
}
