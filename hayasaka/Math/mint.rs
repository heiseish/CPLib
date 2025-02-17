mod modular {
    use std::fmt;
    use std::iter::Sum;
    use std::ops::*;

    pub type mint1000000007 = MInt<1_000_000_007>;
    pub type mint998244353 = MInt<998244353>;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MInt<const MOD: i64>(i64);

    impl<const MOD: i64> fmt::Display for MInt<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<const MOD: i64> fmt::Debug for MInt<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    const fn add<const MOD: i64>(mut a: i64, mut b: i64) -> i64 {
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

    const fn mult<const MOD: i64>(a: i64, b: i64) -> i64 {
        ((a % MOD) * (b % MOD)) % MOD
    }

    const fn bin_pow<const MOD: i64>(mut a: i64, mut b: i64) -> i64 {
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

    const fn bin_pow_u128<const MOD: i64>(mut a: i64, mut b: u128) -> i64 {
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
        pub const ZERO: MInt<MOD> = MInt(0);
        pub const ONE: MInt<MOD> = MInt(1);
        pub const INVALID: MInt<MOD> = MInt(i64::MAX / 4);

        pub const fn new(value: i64) -> Self {
            Self(value % MOD)
        }

        pub const fn pow(self, exp: MInt<MOD>) -> Self {
            Self::new(bin_pow::<MOD>(self.0, exp.0))
        }

        pub const fn pow_u128(self, exp: u128) -> Self {
            Self::new(bin_pow_u128::<MOD>(self.0, exp))
        }

        pub const fn inv(self) -> Self {
            self.pow(Self::new(MOD - 2))
        }
        pub fn init_inv_mod(n: usize) -> Vec<MInt<MOD>> {
            let mut inv = vec![0i64; n];
            inv[0] = 1;
            inv[1] = 1;
            for i in 2..n {
                inv[i] = mult::<MOD>(
                    add::<MOD>(MOD, -MOD / i as i64),
                    inv[(MOD % i as i64) as usize],
                );
            }
            inv.into_iter().map(|x| x.into()).collect()
        }

        pub fn init_fact_mod(n: usize) -> Vec<MInt<MOD>> {
            let mut fact = vec![MInt::<MOD>::new(1); n];
            for i in 1..n {
                fact[i] = fact[i - 1] * i as i64;
            }
            fact
        }

        pub fn init_ifact_mod(n: usize, fact: &[MInt<MOD>]) -> Vec<MInt<MOD>> {
            let mut ifact = vec![MInt::<MOD>::new(0); n];
            ifact[n - 1] = fact[n - 1].inv();
            for i in (0..(n - 1)).rev() {
                ifact[i] = ifact[i + 1] * (i + 1) as i64;
            }
            ifact
        }

        // init (fact, ifact)
        pub fn init_fact(n: usize) -> (Vec<MInt<MOD>>, Vec<MInt<MOD>>) {
            let mut fact = vec![MInt::<MOD>::new(1); n];
            let mut ifact = vec![MInt::<MOD>::new(0); n];
            for i in 1..n {
                fact[i] = fact[i - 1] * i as i64;
            }
            ifact[n - 1] = fact[n - 1].inv();
            for i in (0..(n - 1)).rev() {
                ifact[i] = ifact[i + 1] * (i + 1) as i64;
            }
            (fact, ifact)
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

    impl<const MOD: i64> BitAnd for MInt<MOD> {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self::new(self.0 & rhs.0)
        }
    }

    impl<const MOD: i64> BitAnd<i64> for MInt<MOD> {
        type Output = Self;

        fn bitand(self, rhs: i64) -> Self::Output {
            Self::new(self.0 & rhs)
        }
    }

    impl<const MOD: i64> BitAnd<i32> for MInt<MOD> {
        type Output = Self;

        fn bitand(self, rhs: i32) -> Self::Output {
            Self::new(self.0 & rhs as i64)
        }
    }

    impl<const MOD: i64> BitAnd<usize> for MInt<MOD> {
        type Output = Self;

        fn bitand(self, rhs: usize) -> Self::Output {
            Self::new(self.0 & rhs as i64)
        }
    }

    impl<const MOD: i64> BitOr for MInt<MOD> {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            Self::new(self.0 | rhs.0)
        }
    }

    impl<const MOD: i64> BitOr<i32> for MInt<MOD> {
        type Output = Self;

        fn bitor(self, rhs: i32) -> Self::Output {
            Self::new(self.0 | rhs as i64)
        }
    }

    impl<const MOD: i64> BitOr<i64> for MInt<MOD> {
        type Output = Self;

        fn bitor(self, rhs: i64) -> Self::Output {
            Self::new(self.0 | rhs)
        }
    }

    impl<const MOD: i64> BitOr<usize> for MInt<MOD> {
        type Output = Self;

        fn bitor(self, rhs: usize) -> Self::Output {
            Self::new(self.0 | rhs as i64)
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

    impl<const MOD: i64> From<u128> for MInt<MOD> {
        fn from(val: u128) -> Self {
            Self::new((val % MOD as u128) as i64)
        }
    }

    impl<const MOD: i64> From<i128> for MInt<MOD> {
        fn from(val: i128) -> Self {
            Self::new((val % MOD as i128) as i64)
        }
    }

    impl<const MOD: i64> From<usize> for MInt<MOD> {
        fn from(val: usize) -> Self {
            Self::new(val as i64)
        }
    }

    impl<const MOD: i64> From<MInt<MOD>> for usize {
        fn from(val: MInt<MOD>) -> Self {
            val.0 as usize
        }
    }

    impl<const MOD: i64> Sum<Self> for MInt<MOD> {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
        {
            iter.fold(MInt::<MOD>::ZERO, |a, b| a + b)
        }
    }

    // let (fact, ifact) = mint::init_fact(200_005);
    // let C = modular::choose_curry(&fact, &ifact);
    pub fn choose_curry<'a, const MOD: i64>(
        fact: &'a [MInt<MOD>],
        ifact: &'a [MInt<MOD>],
    ) -> impl Fn(i32, i32) -> MInt<MOD> + 'a {
        |n: i32, c: i32| -> MInt<MOD> {
            if n < 0 || c < 0 || n - c < 0 {
                return MInt::<MOD>::ZERO;
            }
            let n = n as usize;
            let c = c as usize;
            fact[n] * ifact[c] * ifact[n - c]
        }
    }
}
pub type mint = modular::mint998244353;
pub type mint = modular::mint1000000007;
