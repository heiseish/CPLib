mod mint {
    use std::fmt;
    use std::ops::*;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MInt<const MOD: i64>(i64);

    impl<const MOD: i64> fmt::Display for MInt<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    fn add<const MOD: i64>(mut a: i64, mut b: i64) -> i64 {
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

    fn mult<const MOD: i64>(a: i64, b: i64) -> i64 {
        ((a % MOD) * (b % MOD)) % MOD
    }

    fn bin_pow<const MOD: i64>(mut a: i64, mut b: i64) -> i64 {
        a %= MOD;
        let mut ret = 1;
        while b > 0 {
            if (b & 1) > 0 {
                ret = mult::<MOD>(ret, a);
            }
            a = mult::<MOD>(a, a);
            b >>= 1;
        }
        ret
    }

    impl<const MOD: i64> MInt<MOD> {
        pub fn z() -> Self {
            Self::default()
        }

        pub fn new(value: i64) -> Self {
            Self(value % MOD)
        }

        pub fn pow(self, mut exp: MInt<MOD>) -> Self {
            Self::new(bin_pow::<MOD>(self.0, exp.0))
        }

        pub fn inv(self) -> Self {
            self.pow(Self::new(MOD - 2))
        }
    }

    impl<const MOD: i64> Add for MInt<MOD> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(add::<MOD>(self.0, rhs.0))
        }
    }

    impl<const MOD: i64> Add<i64> for MInt<MOD> {
        type Output = Self;

        fn add(self, rhs: i64) -> Self::Output {
            Self::new(add::<MOD>(self.0, rhs))
        }
    }

    impl<const MOD: i64> AddAssign for MInt<MOD> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: i64> AddAssign<i64> for MInt<MOD> {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: i64> Sub for MInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(add::<MOD>(self.0, -rhs.0))
        }
    }

    impl<const MOD: i64> Sub<i64> for MInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: i64) -> Self::Output {
            Self::new(add::<MOD>(self.0, -rhs))
        }
    }

    impl<const MOD: i64> SubAssign for MInt<MOD> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: i64> SubAssign<i64> for MInt<MOD> {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: i64> Neg for MInt<MOD> {
        type Output = Self;

        // Required method
        fn neg(self) -> Self::Output {
            Self::new(add::<MOD>(-self.0, 0))
        }
    }

    impl<const MOD: i64> Mul for MInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(mult::<MOD>(self.0, rhs.0))
        }
    }

    impl<const MOD: i64> Mul<i64> for MInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Self::new(mult::<MOD>(self.0, rhs))
        }
    }

    impl<const MOD: i64> MulAssign<Self> for MInt<MOD> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: i64> MulAssign<i64> for MInt<MOD> {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: i64> Div for MInt<MOD> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MOD: i64> Div<i64> for MInt<MOD> {
        type Output = Self;

        fn div(self, rhs: i64) -> Self::Output {
            self * Self::new(rhs).inv()
        }
    }

    impl<const MOD: i64> DivAssign<Self> for MInt<MOD> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self * rhs.inv();
        }
    }

    impl<const MOD: i64> DivAssign<i64> for MInt<MOD> {
        fn div_assign(&mut self, rhs: i64) {
            *self = *self * Self::new(rhs).inv();
        }
    }

    impl<const MOD: i64> From<i32> for MInt<MOD> {
        fn from(val: i32) -> Self {
            Self::new(val as i64)
        }
    }

    impl<const MOD: i64> From<i64> for MInt<MOD> {
        fn from(val: i64) -> Self {
            Self::new(val)
        }
    }

    impl<const MOD: i64> From<u32> for MInt<MOD> {
        fn from(val: u32) -> Self {
            Self::new(val as i64)
        }
    }

    impl<const MOD: i64> From<u64> for MInt<MOD> {
        fn from(val: u64) -> Self {
            Self::new(val as i64)
        }
    }

    impl<const MOD: i64> From<usize> for MInt<MOD> {
        fn from(val: usize) -> Self {
            Self::new(val as i64)
        }
    }

    pub fn init_inv_mod<const MOD: i64, const N: usize>() -> [MInt<MOD>; N] {
        let mut inv = [0i64; N];
        inv[0] = 1;
        inv[1] = 1;
        for i in 2..N {
            inv[i] = mult::<MOD>(
                add::<MOD>(MOD, -MOD / i as i64),
                inv[(MOD % i as i64) as usize],
            );
        }
        inv.map(|x| x.into())
    }
}
