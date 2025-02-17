fn compress(mut a: Vec<i64>, mi: Option<usize>) -> HashMap<i64, usize> {
    let mut mi = mi.unwrap_or(0);
    let mut ret = HashMap::new();
    for b in a.into_iter().collect::<BTreeSet<_>>().into_iter() {
        if ret.contains_key(&b) {
            continue;
        }
        ret.insert(b, mi);
        mi += 1;
    }
    ret
}

mod fw {
    type FT = i64;
    macro_rules! lsone {
        ($S:tt) => {
            (($S as FT) & -($S as FT))
        };
    }
    pub struct FenwichTree {
        ft: Vec<i64>,
        n: usize,
    }
    impl FenwichTree {
        pub fn new(n: usize) -> Self {
            let ft = vec![0; n + 1];
            Self { ft, n }
        }

        // Input has to be 1-based indexing
        pub fn from_arr(f: &[FT]) -> Self {
            let n = f.len() - 1;
            let mut ft = vec![0; n + 1];
            for i in 1..=n {
                ft[i] += f[i];
                let j = i + lsone!(i) as usize;
                if j <= n {
                    ft[j] += ft[i];
                }
            }
            Self { ft, n }
        }

        #[inline(always)]
        pub fn rsq_s(&self, mut j: FT) -> FT {
            let mut sum = 0;
            while j > 0 {
                sum += self.ft[j as usize];
                j -= lsone!(j);
            }
            sum
        }

        #[inline(always)]
        pub fn rsq(&self, i: FT, j: FT) -> FT {
            self.rsq_s(j) - self.rsq_s(i - 1)
        }

        #[inline(always)]
        pub fn update(&mut self, mut i: usize, v: FT) {
            loop {
                if i > self.n {
                    break;
                }
                self.ft[i] += v;
                i += lsone!(i) as usize;
            }
        }

        #[inline(always)]
        pub fn bit_search(&self, v: FT, bound: usize) -> usize {
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
use fw::*;