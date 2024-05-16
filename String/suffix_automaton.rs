mod sa {
    use std::collections::BTreeMap;

    pub struct SuffixAutomaton {
        pub st: Vec<SAState>,
        pub sz: usize,
        pub last: usize,
    }
    #[derive(Default, Clone)]
    pub struct SAState {
        pub len: usize,
        pub link: i32,
        pub next: BTreeMap<char, usize>,
    }
    impl SuffixAutomaton {
        pub fn from_str(s: &str) -> Self {
            let mut sa = Self::new(s.len());
            for ch in s.chars() {
                sa.add(ch);
            }
            sa
        }
        pub fn new(n: usize) -> Self {
            let mut sa = Self {
                st: vec![],
                sz: 1,
                last: 0,
            };
            for i in 0..(2 * n) {
                sa.st.push(SAState::default());
            }
            sa.st[0].len = 0;
            sa.st[0].link = -1;
            sa
        }
        pub fn add(&mut self, c: char) {
            let cur = self.sz;
            self.sz += 1;
            self.st[cur].len = self.st[self.last].len + 1;
            let mut p = self.last as i32;
            while p != -1 && !self.st[p as usize].next.contains_key(&c) {
                self.st[p as usize].next.insert(c, cur);
                p = self.st[p as usize].link;
            }
            if p == -1 {
                self.st[cur].link = 0;
            } else {
                let pu = p as usize;
                let q = self.st[pu].next[&c];
                if self.st[pu].len + 1 == self.st[q].len {
                    self.st[cur].link = q as i32;
                } else {
                    let clone = self.sz;
                    self.sz += 1;
                    self.st[clone].len = self.st[pu].len + 1;
                    self.st[clone].next = self.st[q].next.clone();
                    self.st[clone].link = self.st[q].link;

                    while p != -1 && *self.st[p as usize].next.get(&c).unwrap() == q {
                        self.st[p as usize].next.insert(c, clone);
                        p = self.st[p as usize].link;
                    }
                    self.st[cur].link = clone as i32;
                    self.st[q].link = self.st[cur].link;
                }
            }
            self.last = cur;
        }

        pub fn count_unique_substring(&self) -> i64 {
            let mut tot = 0;
            for i in 1..self.sz {
                tot += (self.st[i].len - self.st[self.st[i].link as usize].len) as i64;
            }
            tot
        }
    }
}
