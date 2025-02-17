mod mat {
    use crate::mint;
    use core::ops::{Index, IndexMut, Mul, MulAssign};

    #[derive(Clone, Copy)]
    pub struct Matrix<const NR: usize, const NC: usize> {
        pub data: [[mint; NC]; NR],
    }

    impl<const NR: usize, const NC: usize> Default for Matrix<NR, NC> {
        fn default() -> Self {
            Self {
                data: [[mint::ZERO; NC]; NR],
            }
        }
    }

    impl<const R1: usize, const C1: usize, const C2: usize> Mul<Matrix<C1, C2>> for Matrix<R1, C1> {
        type Output = Matrix<R1, C2>;

        fn mul(self, rhs: Matrix<C1, C2>) -> Self::Output {
            let mut ans = [[mint::ZERO; C2]; R1];

            for i in 0..R1 {
                for j in 0..C2 {
                    for k in 0..C1 {
                        ans[i][j] += self.data[i][k] * rhs.data[k][j];
                    }
                }
            }
            Matrix::<R1, C2> { data: ans }
        }
    }

    impl<const NR: usize, const NC: usize> Index<usize> for Matrix<NR, NC> {
        type Output = [mint];
        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<const NR: usize, const NC: usize> IndexMut<usize> for Matrix<NR, NC> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }

    impl<const N: usize> MulAssign<Matrix<N, N>> for Matrix<N, N> {
        fn mul_assign(&mut self, rhs: Matrix<N, N>) {
            *self = *self * rhs;
        }
    }

    impl<const NR: usize, const NC: usize> Matrix<NR, NC> {
        pub const fn new() -> Self {
            Self {
                data: [[mint::ZERO; NC]; NR],
            }
        }
    }

    impl<const N: usize> Matrix<N, N> {
        pub fn pow(mut self, mut p: i64) -> Self {
            let mut ans = Self {
                data: [[mint::ZERO; N]; N],
            };
            for i in 0..N {
                for j in 0..N {
                    ans.data[i][j] = if i == j { mint::ONE } else { mint::ZERO };
                }
            }
            while p > 0 {
                if p & 1 > 0 {
                    ans *= self;
                }
                self = self * self;
                p >>= 1;
            }
            ans
        }
    }
}
pub type matrix<const N: usize> = mat::Matrix<N, N>;
