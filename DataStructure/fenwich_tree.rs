mod fenwich {
    macro_rules! lsone {
        ($S:tt) => {
            (($S as i32) & -($S as i32))
        };
    }
    pub struct FenwichTree {
        ft: Vec<u32>,
        n: usize,
    }
    impl FenwichTree {
        //! Input has to be 1-based indexing
        pub fn new(f: &[u8]) -> Self {
            let n = f.len() - 1;
            let mut ft = vec![0u32; n + 1];
            for i in 1..=n {
                ft[i] += f[i] as u32;
                let j = i + lsone!(i) as usize;
                if j <= n {
                    ft[j] += ft[i];
                }
            }
            Self { ft, n }
        }

        #[inline(always)]
        pub fn rsq_s(&self, mut j: i32) -> u32 {
            let mut sum = 0;
            while j > 0 {
                sum += self.ft[j as usize];
                j -= lsone!(j);
            }
            sum
        }

        #[inline(always)]
        pub fn rsq(&self, i: i32, j: i32) -> u32 {
            self.rsq_s(j) - self.rsq_s(i - 1)
        }

        #[inline(always)]
        pub fn update(&mut self, mut i: usize, v: u32) {
            loop {
                if i > self.n {
                    break;
                }
                self.ft[i] += v;
                i += lsone!(i) as usize;
            }
        }

        #[inline(always)]
        pub fn bit_search(&self, v: u32, bound: usize) -> usize {
            let mut sum = 0;
            let mut pos = 0;
            let logn = (self.n as f32).log2().floor() as usize + 1;
            for i in (0..=logn).rev() {
                if pos + (1 << i) < bound && sum + self.ft[pos + (1 << i)] >= v {
                    sum += self.ft[pos + (1 << i)];
                    pos += 1 << i;
                }
            }
            pos
        }
    }
}
