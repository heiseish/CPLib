const MOD: i64 = 1_000_000_007;

trait Lsone {
    fn lsone(self) -> Self;
}

impl Lsone for i64 {
    fn lsone(self) -> Self {
        self & -self
    }
}
impl Lsone for i32 {
    fn lsone(self) -> Self {
        self & -self
    }
}
impl Lsone for usize {
    fn lsone(self) -> Self {
        ((self as i64) & -(self as i64)) as Self
    }
}

fn init_inv_mod<const N: usize>() -> [i64; N] {
    let mut inv = [0i64; N];
    inv[0] = 1;
    inv[1] = 1;
    for i in 2..N {
        inv[i] = mult(add(MOD, -MOD / i as i64), inv[(MOD % i as i64) as usize]);
    }
    inv
}
