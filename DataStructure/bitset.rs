mod bitset {
    const TRUE: &bool = &true;
    const FALSE: &bool = &false;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// Fixed sized bitset
    pub struct BitSet {
        buf: Vec<u64>,
        size: usize,
    }

    impl BitSet {
        /// Construct a new, zero bitset with specified capacity.
        /// This method allocates O(size) bits
        pub fn new(size: usize) -> BitSet {
            BitSet {
                buf: vec![0; (size + 63) / 64],
                size,
            }
        }

        /// Set i-th bit to `b`
        #[inline]
        pub fn set(&mut self, i: usize, b: bool) {
            assert!(i < self.size);
            if b {
                self.buf[i >> 6] |= 1 << (i & 63);
            } else {
                self.buf[i >> 6] &= !(1 << (i & 63));
            }
        }

        /// Get the size of bits
        #[inline]
        pub fn size(&self) -> usize {
            self.size
        }

        /// Get the number of ones
        #[inline]
        pub fn count_ones(&self) -> u32 {
            self.buf.iter().map(|x| x.count_ones()).sum()
        }

        /// Get the number of zeros
        #[inline]
        pub fn count_zeros(&self) -> u32 {
            self.size as u32 - self.count_ones()
        }

        /// Faster left shift and or
        ///
        /// `bitset | (bitset << x)`
        #[inline]
        pub fn shl_or(&mut self, x: usize) {
            let q = x >> 6;
            let r = x & 63;

            if q >= self.buf.len() {
                return;
            }

            if r == 0 {
                for i in (q..self.buf.len()).rev() {
                    *unsafe { self.buf.get_unchecked_mut(i) } |=
                        *unsafe { self.buf.get_unchecked(i - q) };
                }
            } else {
                for i in (q + 1..self.buf.len()).rev() {
                    *unsafe { self.buf.get_unchecked_mut(i) } |=
                        (unsafe { self.buf.get_unchecked(i - q) } << r)
                            | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));
                }
                *unsafe { self.buf.get_unchecked_mut(q) } |=
                    unsafe { self.buf.get_unchecked(0) } << r;
            }

            self.chomp();
        }

        /// Get inner buffer
        #[inline]
        pub fn buffer(&self) -> &[u64] {
            &self.buf
        }

        /// Get inner buffer with mutable reference
        #[inline]
        pub fn buffer_mut(&mut self) -> &mut [u64] {
            &mut self.buf
        }

        /// Set tailing bits in inner buffer whose index are greater than size to `0`
        #[inline]
        pub fn chomp(&mut self) {
            let r = self.size & 63;
            if r != 0 {
                if let Some(x) = self.buf.last_mut() {
                    let d = 64 - r;
                    *x = (*x << d) >> d;
                }
            }
        }
    }

    impl std::ops::Index<usize> for BitSet {
        type Output = bool;
        #[inline]
        fn index(&self, index: usize) -> &bool {
            assert!(index < self.size);
            [FALSE, TRUE][(self.buf[index >> 6] >> (index & 63)) as usize & 1]
        }
    }

    impl std::ops::ShlAssign<usize> for BitSet {
        #[inline]
        fn shl_assign(&mut self, x: usize) {
            let q = x >> 6;
            let r = x & 63;

            if q >= self.buf.len() {
                for x in &mut self.buf {
                    *x = 0;
                }
                return;
            }

            if r == 0 {
                for i in (q..self.buf.len()).rev() {
                    *unsafe { self.buf.get_unchecked_mut(i) } =
                        *unsafe { self.buf.get_unchecked(i - q) };
                }
            } else {
                for i in (q + 1..self.buf.len()).rev() {
                    *unsafe { self.buf.get_unchecked_mut(i) } =
                        (unsafe { self.buf.get_unchecked(i - q) } << r)
                            | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));
                }
                *unsafe { self.buf.get_unchecked_mut(q) } =
                    unsafe { self.buf.get_unchecked(0) } << r;
            }

            for x in &mut self.buf[..q] {
                *x = 0;
            }

            self.chomp();
        }
    }

    impl std::ops::Shl<usize> for BitSet {
        type Output = Self;

        #[inline]
        fn shl(mut self, x: usize) -> Self {
            self <<= x;
            self
        }
    }

    impl<'a> std::ops::Shl<usize> for &'a BitSet {
        type Output = BitSet;
        #[inline]
        fn shl(self, x: usize) -> Self::Output {
            let mut result = self.clone();
            result <<= x;
            result
        }
    }

    impl std::ops::ShrAssign<usize> for BitSet {
        #[inline]
        fn shr_assign(&mut self, x: usize) {
            let q = x >> 6;
            let r = x & 63;

            if q >= self.buf.len() {
                for x in &mut self.buf {
                    *x = 0;
                }
                return;
            }

            if r == 0 {
                for i in 0..self.buf.len() - q {
                    *unsafe { self.buf.get_unchecked_mut(i) } =
                        *unsafe { self.buf.get_unchecked(i + q) };
                }
            } else {
                for i in 0..self.buf.len() - q - 1 {
                    *unsafe { self.buf.get_unchecked_mut(i) } =
                        (unsafe { self.buf.get_unchecked(i + q) } >> r)
                            | (unsafe { self.buf.get_unchecked(i + q + 1) } << (64 - r));
                }
                let len = self.buf.len();
                *unsafe { self.buf.get_unchecked_mut(len - q - 1) } =
                    unsafe { self.buf.get_unchecked(len - 1) } >> r;
            }

            let len = self.buf.len();
            for x in &mut self.buf[len - q..] {
                *x = 0;
            }
        }
    }

    impl std::ops::Shr<usize> for BitSet {
        type Output = Self;

        #[inline]
        fn shr(mut self, x: usize) -> Self {
            self >>= x;
            self
        }
    }

    impl<'a> std::ops::Shr<usize> for &'a BitSet {
        type Output = BitSet;

        #[inline]
        fn shr(self, x: usize) -> Self::Output {
            let mut result = self.clone();
            result >>= x;
            result
        }
    }

    impl<'a> std::ops::BitAndAssign<&'a BitSet> for BitSet {
        #[inline]
        fn bitand_assign(&mut self, rhs: &'a Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a &= *b;
            }
        }
    }

    impl<'a> std::ops::BitAndAssign<BitSet> for BitSet {
        #[inline]
        fn bitand_assign(&mut self, rhs: Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a &= *b;
            }
        }
    }

    impl<'a> std::ops::BitAnd<&'a BitSet> for BitSet {
        type Output = Self;
        #[inline]
        fn bitand(mut self, rhs: &'a Self) -> Self::Output {
            self &= rhs;
            self
        }
    }

    impl<'a, 'b> std::ops::BitAnd<&'b BitSet> for &'a BitSet {
        type Output = BitSet;
        #[inline]
        fn bitand(self, rhs: &'b BitSet) -> Self::Output {
            let mut result = self.clone();
            result &= rhs;
            result
        }
    }

    impl<'a> std::ops::BitOrAssign<&'a BitSet> for BitSet {
        #[inline]
        fn bitor_assign(&mut self, rhs: &'a Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a |= *b;
            }
            self.chomp();
        }
    }

    impl std::ops::BitOrAssign<BitSet> for BitSet {
        #[inline]
        fn bitor_assign(&mut self, rhs: Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a |= *b;
            }
            self.chomp();
        }
    }

    impl<'a> std::ops::BitOr<&'a BitSet> for BitSet {
        type Output = Self;
        #[inline]
        fn bitor(mut self, rhs: &'a Self) -> Self::Output {
            self |= rhs;
            self
        }
    }

    impl std::ops::BitOr<BitSet> for BitSet {
        type Output = Self;
        #[inline]
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }

    impl<'a, 'b> std::ops::BitOr<&'b BitSet> for &'a BitSet {
        type Output = BitSet;
        #[inline]
        fn bitor(self, rhs: &'b BitSet) -> Self::Output {
            let mut result = self.clone();
            result |= rhs;
            result
        }
    }

    impl<'a> std::ops::BitXorAssign<&'a BitSet> for BitSet {
        #[inline]
        fn bitxor_assign(&mut self, rhs: &'a Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a ^= *b;
            }
            self.chomp();
        }
    }

    impl std::ops::BitXorAssign<BitSet> for BitSet {
        #[inline]
        fn bitxor_assign(&mut self, rhs: Self) {
            for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
                *a ^= *b;
            }
            self.chomp();
        }
    }

    impl<'a> std::ops::BitXor<&'a BitSet> for BitSet {
        type Output = Self;
        #[inline]
        fn bitxor(mut self, rhs: &'a Self) -> Self {
            self ^= rhs;
            self
        }
    }

    impl<'a, 'b> std::ops::BitXor<&'b BitSet> for &'a BitSet {
        type Output = BitSet;
        #[inline]
        fn bitxor(self, rhs: &'b BitSet) -> Self::Output {
            let mut result = self.clone();
            result ^= rhs;
            result
        }
    }

    impl std::ops::Not for BitSet {
        type Output = Self;
        #[inline]
        fn not(mut self) -> Self::Output {
            for x in &mut self.buf {
                *x = !*x;
            }
            self.chomp();
            self
        }
    }

    impl<'a> std::ops::Not for &'a BitSet {
        type Output = BitSet;
        #[inline]
        fn not(self) -> Self::Output {
            !self.clone()
        }
    }
}
