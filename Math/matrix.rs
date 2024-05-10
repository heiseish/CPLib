const MOD: i64 = 1_000_000_007;

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

#[derive(Clone, Copy)]
struct Matrix<const NR: usize, const NC: usize> {
    pub data: [[i64; NC]; NR],
}

impl<const NR: usize, const NC: usize> Default for Matrix<NR, NC> {
    fn default() -> Self {
        Self {
            data: [[0i64; NC]; NR],
        }
    }
}

impl<const R1: usize, const C1: usize, const C2: usize> Mul<Matrix<C1, C2>> for Matrix<R1, C1> {
    type Output = Matrix<R1, C2>;

    fn mul(self, rhs: Matrix<C1, C2>) -> Self::Output {
        let mut ans = [[0i64; C2]; R1];

        for i in 0..R1 {
            for j in 0..C2 {
                for k in 0..C1 {
                    ans[i][j] = add(ans[i][j], mult(self.data[i][k], rhs.data[k][j]));
                }
            }
        }
        Matrix::<R1, C2> { data: ans }
    }
}

impl<const NR: usize, const NC: usize> Matrix<NR, NC> {
    pub fn new() -> Self {
        Self {
            data: [[0i64; NC]; NR],
        }
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn pow(mut self, mut p: i64) -> Self {
        let mut ans = Self {
            data: [[0i64; N]; N],
        };
        for i in 0..N {
            for j in 0..N {
                ans.data[i][j] = (i == j) as i64;
            }
        }
        while p > 0 {
            if p & 1 > 0 {
                ans = ans * self;
            }
            self = self * self;
            p >>= 1;
        }
        ans
    }
}
