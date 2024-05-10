pub struct LinearDophantine;
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

fn add(mut a: i64, mut b: i64) -> i64 {
    if a < 0 {
        a += MOD;
    }
    a %= MOD;
    if b < 0 {
        b += MOD;
    }
    b %= MOD;
    (a + b) % MOD
}

fn mult(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn bin_pow(mut a: i64, mut b: i64) -> i64 {
    a %= MOD;
    let mut ret = 1;
    while b > 0 {
        if (b & 1) > 0 {
            ret = mult(ret, a);
        }
        a = mult(a, a);
        b >>= 1;
    }
    ret
}

impl LinearDophantine {
    fn gcd(&self, a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }
        let mut x1 = 0;
        let mut y1 = 0;
        let d = self.gcd(b, a % b, &mut x1, &mut y1);
        *x = y1;
        *y = x1 - y1 * (a / b);
        return d;
    }

    pub fn find_any_solution(&self, a: i64, b: i64, c: i64, x: &mut i64, y: &mut i64) -> bool {
        let g = self.gcd(a.abs(), b.abs(), x, y);
        if c % g != 0 {
            return false;
        }

        *x *= c / g;
        *y *= c / g;
        if a < 0 {
            *x = -*x;
        }
        if b < 0 {
            *y = -*y;
        }

        /// update x and y to satisfy requirement here
        while *x < 0 || *y < 0 {
            if b > 0 && a < 0 {
                let temp = {
                    if *x < 0 {
                        ((*x as f32) / (b as f32)).ceil() as i64
                    } else {
                        ((*y as f32) / (a as f32)).ceil() as i64
                    }
                };
                *x += b * temp;
                *y -= a * temp;
            } else {
                let temp = {
                    if *x < 0 {
                        ((*x as f32) / (b as f32)).ceil() as i64
                    } else {
                        ((*y as f32) / (a as f32)).ceil() as i64
                    }
                };
                *x -= b * temp;
                *y += a * temp;
            }
        }
        return true;
    }
}
