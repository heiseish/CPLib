fn median(a: i32, b: i32, c: i32) -> i32 {
    a.min(b).max(a.max(b).min(c))
}
