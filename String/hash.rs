mod str_hash {
    type HashT = u64;

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
                p_pow[i] = (p_pow[i - 1] * P as HashT) % M;
            }
            Self { p_pow, h: vec![] }
        }

        pub fn hash(&mut self, s: &[char]) -> usize {
            let tag = self.h.len();
            let n = s.len();
            self.h.push(vec![0; n + 1]);
            for i in 0..n {
                self.h[tag][i + 1] =
                    (self.h[tag][i] + (s[i] as HashT - CH as HashT + 1) * self.p_pow[i]) % M;
            }
            tag
        }

        pub fn compute_with_len(&mut self, tag: usize, first: usize, len: usize) -> HashT {
            let h = &self.h[tag];
            let mut res = (h[first + len] + M - h[first]) % M;
            (res * self.p_pow[N - first]) % M
        }

        pub fn compute_with_bound(&mut self, tag: usize, first: usize, last: usize) -> HashT {
            let h = &self.h[tag];
            let mut res = (h[last + 1] + M - h[first]) % M;
            (res * self.p_pow[N - first]) % M
        }

        pub fn compute_max_len(&mut self, tag: usize) -> HashT {
            self.h[tag][self.h[tag].len() - 1]
        }
    }
}

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
