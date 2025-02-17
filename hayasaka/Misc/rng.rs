mod rng {
    use std::time::{SystemTime, UNIX_EPOCH};
    pub type SmallRng = Xoshiro256PlusPlus;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Xoshiro256PlusPlus {
        s: [u64; 4],
    }

    impl Default for Xoshiro256PlusPlus {
        fn default() -> Self {
            Self::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis() as u64,
            )
        }
    }
    impl Xoshiro256PlusPlus {
        /// Construct a new RNG from a 64-bit seed.
        pub fn new(mut state: u64) -> Self {
            const PHI: u64 = 0x9e3779b97f4a7c15;
            let mut seed = <[u64; 4]>::default();
            for chunk in &mut seed {
                state = state.wrapping_add(PHI);
                let mut z = state;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                z = z ^ (z >> 31);
                *chunk = z;
            }
            Self { s: seed }
        }

        /// Generate a random `u32`.
        #[inline]
        pub fn next_u32(&mut self) -> u32 {
            (self.next_u64() >> 32) as u32
        }

        /// Generate a random `u64`.
        #[inline]
        pub fn next_u64(&mut self) -> u64 {
            let result_plusplus = self.s[0]
                .wrapping_add(self.s[3])
                .rotate_left(23)
                .wrapping_add(self.s[0]);

            let t = self.s[1] << 17;

            self.s[2] ^= self.s[0];
            self.s[3] ^= self.s[1];
            self.s[1] ^= self.s[2];
            self.s[0] ^= self.s[3];

            self.s[2] ^= t;

            self.s[3] = self.s[3].rotate_left(45);

            result_plusplus
        }
    }
}
