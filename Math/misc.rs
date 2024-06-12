pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        a %= b;
        (a, b) = (b, a);
    }
    a
}
pub fn lcm(a: i32, b: i32) -> i32 {
    return a / gcd(a, b) * b;
}

fn lcm_seq(a: &VecDeque<i32>) -> i32 {
    if a.len() == 0 {
        return 0;
    }
    let mut res = a[0];
    for i in 1..a.len() {
        res = lcm(res, a[i]);
    }
    res
}
