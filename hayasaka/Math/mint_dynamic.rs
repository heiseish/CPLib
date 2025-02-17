mod modular {
    use std::fmt;
    use std::iter::Sum;
    use std::ops::*;
    pub type Mint1000000007 = MInt;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MInt {
        value: i64,
        modulus: i64,
    }

    impl fmt::Display for MInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl fmt::Debug for MInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl MInt {
        pub const fn new(value: i64, modulus: i64) -> Self {
            Self {
                value: value % modulus,
                modulus,
            }
        }

        // pub fn pow(self, exp: MInt) -> Self {
        //     self.pow_internal(exp.value)
        // }

        // fn pow_internal(mut self, exp: i64) -> Self {
        //     let mut ret = MInt::ONE;
        //     let mut base = self.value;
        //     let mut exp = exp;
        //     while exp > 0 {
        //         if (exp & 1) == 1 {
        //             ret = ret * base % self.modulus;
        //         }
        //         base = base * base % self.modulus;
        //         exp >>= 1;
        //     }
        //     ret
        // }

        // pub fn inv(self) -> Self {
        //     self.pow_internal(self.modulus - 2)
        // }

        // pub fn init_inv_mod(n: usize, modulus: i64) -> Vec<MInt> {
        //     let mut inv = vec![MInt::ZERO; n];
        //     inv[0] = MInt::new(1, modulus);
        //     inv[1] = MInt::new(1, modulus);
        //     for i in 2..n {
        //         inv[i] = MInt::new(
        //             (modulus - modulus / i as i64) * inv[(modulus % i as i64) as usize].value
        //                 % modulus,
        //             modulus,
        //         );
        //     }
        //     inv
        // }

        // pub fn init_fact_mod(n: usize, modulus: i64) -> Vec<MInt> {
        //     let mut fact = vec![MInt::ONE; n];
        //     for i in 1..n {
        //         fact[i] = fact[i - 1] * i as i64;
        //     }
        //     fact
        // }

        // pub fn init_ifact_mod(n: usize, modulus: i64, fact: &[MInt]) -> Vec<MInt> {
        //     let mut ifact = vec![MInt::ZERO; n];
        //     ifact[n - 1] = fact[n - 1].inv();
        //     for i in (0..(n - 1)).rev() {
        //         ifact[i] = ifact[i + 1] * (i + 1) as i64;
        //     }
        //     ifact
        // }

        // pub fn init_fact(n: usize, modulus: i64) -> (Vec<MInt>, Vec<MInt>) {
        //     let mut fact = vec![MInt::ONE; n];
        //     let mut ifact = vec![MInt::ZERO; n];
        //     for i in 1..n {
        //         fact[i] = fact[i - 1] * i as i64;
        //     }
        //     ifact[n - 1] = fact[n - 1].inv();
        //     for i in (0..(n - 1)).rev() {
        //         ifact[i] = ifact[i + 1] * (i + 1) as i64;
        //     }
        //     (fact, ifact)
        // }
    }

    impl Add for MInt {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            assert_eq!(self.modulus, rhs.modulus);
            Self::new((self.value + rhs.value) % self.modulus, self.modulus)
        }
    }

    impl Add<i64> for MInt {
        type Output = Self;

        fn add(self, rhs: i64) -> Self::Output {
            Self::new((self.value + rhs) % self.modulus, self.modulus)
        }
    }

    impl BitAnd for MInt {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self::new(self.value & rhs.value, self.modulus)
        }
    }

    impl BitAnd<i64> for MInt {
        type Output = Self;

        fn bitand(self, rhs: i64) -> Self::Output {
            Self::new(self.value & rhs, self.modulus)
        }
    }

    impl BitAnd<usize> for MInt {
        type Output = Self;

        fn bitand(self, rhs: usize) -> Self::Output {
            Self::new(self.value & rhs as i64, self.modulus)
        }
    }

    impl BitOr for MInt {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            Self::new(self.value | rhs.value, self.modulus)
        }
    }

    impl BitOr<i64> for MInt {
        type Output = Self;

        fn bitor(self, rhs: i64) -> Self::Output {
            Self::new(self.value | rhs, self.modulus)
        }
    }

    impl BitOr<usize> for MInt {
        type Output = Self;

        fn bitor(self, rhs: usize) -> Self::Output {
            Self::new(self.value | rhs as i64, self.modulus)
        }
    }

    impl AddAssign for MInt {
        fn add_assign(&mut self, rhs: Self) {
            assert_eq!(self.modulus, rhs.modulus);
            *self = *self + rhs;
        }
    }

    impl AddAssign<i64> for MInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }

    impl Sub for MInt {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            assert_eq!(self.modulus, rhs.modulus);
            Self::new(
                (self.value - rhs.value + self.modulus) % self.modulus,
                self.modulus,
            )
        }
    }

    impl Sub<i64> for MInt {
        type Output = Self;

        fn sub(self, rhs: i64) -> Self::Output {
            Self::new(
                (self.value - rhs + self.modulus) % self.modulus,
                self.modulus,
            )
        }
    }

    impl SubAssign for MInt {
        fn sub_assign(&mut self, rhs: Self) {
            assert_eq!(self.modulus, rhs.modulus);
            *self = *self - rhs;
        }
    }

    impl SubAssign<i64> for MInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }

    impl Neg for MInt {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new((-self.value + self.modulus) % self.modulus, self.modulus)
        }
    }

    impl Mul for MInt {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            assert_eq!(self.modulus, rhs.modulus);
            Self::new(self.value * rhs.value % self.modulus, self.modulus)
        }
    }

    impl Mul<i64> for MInt {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Self::new(self.value * rhs % self.modulus, self.modulus)
        }
    }

    impl MulAssign<Self> for MInt {
        fn mul_assign(&mut self, rhs: Self) {
            assert_eq!(self.modulus, rhs.modulus);
            *self = *self * rhs;
        }
    }

    impl MulAssign<i64> for MInt {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs;
        }
    }

    // impl Div for MInt {
    //     type Output = Self;

    //     fn div(self, rhs: Self) -> Self::Output {
    //         assert_eq!(self.modulus, rhs.modulus);
    //         self * rhs.inv()
    //     }
    // }

    // impl Div<i64> for MInt {
    //     type Output = Self;

    //     fn div(self, rhs: i64) -> Self::Output {
    //         self * MInt::new(rhs, self.modulus).inv()
    //     }
    // }

    // impl DivAssign<Self> for MInt {
    //     fn div_assign(&mut self, rhs: Self) {
    //         assert_eq!(self.modulus, rhs.modulus);
    //         *self = *self * rhs.inv();
    //     }
    // }

    // impl DivAssign<i64> for MInt {
    //     fn div_assign(&mut self, rhs: i64) {
    //         *self = *self * MInt::new(rhs, self.modulus).inv();
    //     }
    // }
}
pub type mint = modular::MInt;
