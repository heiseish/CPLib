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
