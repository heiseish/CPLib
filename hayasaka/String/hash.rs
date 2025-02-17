// courtesy of tourist
mod hayasaka {
    mod rolling61 {
        const MD: u64 = (1 << 61) - 1;
        const STEP: u64 = 31; // (MD >> 2) + rng.gen_range(0..(MD >> 1))

        #[inline]
        fn addmod(a: u64, b: u64) -> u64 {
            let mut a = a + b;
            if a >= MD {
                a -= MD;
            }
            a
        }

        #[inline]
        fn submod(a: u64, b: u64) -> u64 {
            let mut a = a + MD - b;
            if a >= MD {
                a -= MD;
            }
            a
        }

        #[inline]
        fn mulmod(a: u64, b: u64) -> u64 {
            let l1 = a as u32 as u64;
            let h1 = (a >> 32) as u64;
            let l2 = b as u32 as u64;
            let h2 = (b >> 32) as u64;
            let l = l1 * l2;
            let m = l1 * h2 + l2 * h1;
            let h = h1 * h2;
            let ret = (l & MD) + (l >> 61) + (h << 3) + (m >> 29) + ((m << 35) >> 3) + 1;
            let ret = (ret & MD) + (ret >> 61);
            let ret = (ret & MD) + (ret >> 61);
            ret - 1
        }

        pub struct Hash61 {
            pw: Vec<u64>,
            pref: Vec<u64>,
            n: usize,
        }

        impl Hash61 {
            pub fn new<T: AsRef<[char]>>(s: T) -> Self {
                let s = s.as_ref();
                let n = s.len();

                let mut pw = vec![1];
                for i in 1..=n {
                    let next_pw = mulmod(pw[i - 1], STEP);
                    pw.push(next_pw);
                }

                let mut pref = vec![0; n + 1];
                pref[0] = 1;
                for i in 0..n {
                    pref[i + 1] = addmod(mulmod(pref[i], STEP), s[i] as u64);
                }

                Hash61 { pw, pref, n }
            }

            pub fn hash(&self, from: usize, to: usize) -> u64 {
                assert!(from <= to && to < self.n);
                submod(
                    self.pref[to + 1],
                    mulmod(self.pref[from], self.pw[to - from + 1]),
                )
            }
        }
    }
}
use hayasaka::rolling61::Hash61;

// CP-algorithm
mod hayasaka {
    type HashT = u64;
    type HashT2 = u128;

    // Quite often the above mentioned polynomial hash is good enough, and no collisions will happen during tests. Remember, the probability that collision happens is only
    // $\approx \frac{1}{m}$ . For
    // $m = 10^9 + 9$  the probability is
    // $\approx 10^{-9}$  which is quite low. But notice, that we only did one comparison. What if we compared a string
    // $s$  with
    // $10^6$  different strings. The probability that at least one collision happens is now
    // $\approx 10^{-3}$ . And if we want to compare
    // $10^6$  different strings with each other (e.g. by counting how many unique strings exists), then the probability of at least one collision happening is already
    // $\approx 1$ . It is pretty much guaranteed that this task will end with a collision and returns the wrong result.

    // There is a really easy trick to get better probabilities. We can just compute two different hashes for each string (by using two different
    // $p$ , and/or different
    // $m$ , and compare these pairs instead. If
    // $m$  is about
    // $10^9$  for each of the two hash functions than this is more or less equivalent as having one hash function with
    // $m \approx 10^{18}$ . When comparing
    // $10^6$  strings with each other, the probability that at least one collision happens is now reduced to
    // $\approx 10^{-6}$ .
    pub struct StringHasher<
        const N: usize,
        const CH: char = 'a',
        const P: usize = 31,
        const M: u64 = 1_000_000_009,
    > {
        p_pow: Vec<HashT>,
        h: Vec<Vec<HashT>>,
    }

    impl<const N: usize, const CH: char, const P: usize, const M: HashT> StringHasher<N, CH, P, M> {
        pub fn new() -> Self {
            let mut p_pow = vec![0; N + 1];
            p_pow[0] = 1;
            for i in 1..=N {
                p_pow[i] = ((p_pow[i - 1] as HashT2 * P as HashT2) % M as HashT2) as HashT;
            }
            Self { p_pow, h: vec![] }
        }

        pub fn hash(&mut self, s: &[char]) -> usize {
            let tag = self.h.len();
            let n = s.len();
            self.h.push(vec![0; n + 1]);
            for i in 0..n {
                self.h[tag][i + 1] = self.h[tag][i]
                    + (((s[i] as HashT2 - CH as HashT2 + 1) * self.p_pow[i] as HashT2)
                        % M as HashT2) as HashT;
            }
            tag
        }

        pub fn compute_with_len(&mut self, tag: usize, first: usize, len: usize) -> HashT {
            let h = &self.h[tag];
            let mut res = (h[first + len] + M - h[first]) % M;
            ((res as HashT2 * self.p_pow[N - first] as HashT2) % M as HashT2) as HashT
        }

        pub fn compute_with_bound(&mut self, tag: usize, first: usize, last: usize) -> HashT {
            let h = &self.h[tag];
            let mut res = (h[last + 1] + M - h[first]) % M;
            ((res as HashT2 * self.p_pow[N - first] as HashT2) % M as HashT2) as HashT
        }

        pub fn compute_max_len(&mut self, tag: usize) -> HashT {
            self.h[tag][self.h[tag].len() - 1]
        }
    }
}
use hayasaka::*;

fn solve<R: BufRead, W: Write>(mut input: InputReader<R>, mut output: W) {
    // read in int/float
    let mut hasher = str_hash::StringHasher::<200_005, '0', 2>::new();

    'outer: for _ in 0..(input.next::<usize>()) {
        let n: usize = input.next();
        let k: usize = input.next();
        let mut s = String::new();
        input.read_line(&mut s);
        let s = s.chars().collect::<Vec<char>>();

        if n == k {
            if s.into_iter().collect::<HashSet<_>>().len() != 1 {
                write!(output, "-1\n");
            } else {
                write!(output, "1\n");
            }
            continue 'outer;
        }

        let mut atts = vec![vec![]; 2];
        for i in 0..k {
            atts[0].push('0');
            atts[1].push('1');
        }
        let mut flag = 1;
        while atts[0].len() != n {
            for i in 0..k {
                atts[flag].push('0');
                atts[flag ^ 1].push('1');
            }
            flag ^= 1;
        }
        for i in 0..2 {
            let mut tmp = atts[i].clone();
            atts[i].append(&mut tmp);
        }
        // should have the 000111000111 pattern now
        let mut rs = s.clone();
        rs.reverse();

        let tags = hasher.hash(&s);
        let tagrs = hasher.hash(&rs);
        let mut taghats = [0; 2];

        for i in 0..2 {
            taghats[i] = hasher.hash(&atts[i]);
        }

        for l in 1..=n {
            for j in 0..2 {
                let mut can = true;
                let p = n - l;
                // 1st half
                can &= hasher.compute_with_len(tagrs, n - p, p)
                    == hasher.compute_with_len(taghats[j], l, p);

                // 2nd half
                can &= hasher.compute_with_len(tags, p, l)
                    == hasher.compute_with_len(taghats[j], l + p, l);

                if can {
                    write!(output, "{}\n", p);
                    continue 'outer;
                }
            }
        }

        write!(output, "-1\n");
    }
}
