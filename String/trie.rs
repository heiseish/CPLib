mod trie {
    const ALPHABET_SIZE: usize = 26;

    #[derive(Default, Copy, Clone)]
    pub struct TrieNode {
        link: usize,
        sz: i64,
    }

    impl TrieNode {
        pub fn no_link(&self) -> bool {
            self.link == 0
        }
    }

    #[derive(Default, Clone)]
    pub struct Trie {
        trie: Vec<[TrieNode; ALPHABET_SIZE]>,
        total_idx: usize,
    }

    impl Trie {
        pub fn new(n: usize) -> Self {
            Self {
                trie: vec![[TrieNode::default(); ALPHABET_SIZE]; n + 3],
                total_idx: 2,
            }
        }

        pub fn insert(&mut self, s: &str) {
            let mut cur = 1;
            for ch in s.chars() {
                let idx = (ch as u8 - b'a') as usize;
                if self.trie[cur][idx].no_link() {
                    self.trie[cur][idx].link = self.total_idx;
                    self.total_idx += 1;
                }
                cur = self.trie[cur][idx].link;
                self.trie[cur][idx].sz += 1;
            }
        }

        pub fn search(&self, s: &str) -> i64 {
            let mut ret = 0;
            let mut cur = 1;
            for ch in s.chars() {
                let idx = (ch as u8 - b'a') as usize;
                if self.trie[cur][idx].no_link() {
                    return ret;
                }
                cur = self.trie[cur][idx].link;
                ret += self.trie[cur][idx].sz;
            }
            ret
        }
    }

    type IntType = u32;

    #[derive(Default, Clone)]
    pub struct IntTrie {
        l: usize,
        c: usize,
        node: Vec<[usize; 2]>,
        cnt: Vec<usize>,
    }

    impl IntTrie {
        pub fn new(n: usize) -> Self {
            let num_bits = std::mem::size_of::<IntType>() * 8;
            Self {
                l: num_bits,
                c: 1,
                node: vec![[0, 0]; (num_bits + 2) * n + 3],
                cnt: vec![0; (num_bits + 2) * n + 3],
            }
        }

        pub fn add(&mut self, x: IntType) {
            let mut cur = 0;
            for i in (0..self.l).rev() {
                let has_bit = ((x >> i) & 1) as usize;
                if self.node[cur][has_bit] == 0 {
                    self.node[cur][has_bit] = self.c;
                    self.c += 1;
                }
                cur = self.node[cur][has_bit];
                self.cnt[cur] += 1;
            }
        }

        pub fn remove(&mut self, x: IntType) {
            let mut cur = 0;
            for i in (0..self.l).rev() {
                let has_bit = ((x >> i) & 1) as usize;
                if self.node[cur][has_bit] == 0 {
                    self.node[cur][has_bit] = self.c;
                    self.c += 1;
                }
                cur = self.node[cur][has_bit];
                self.cnt[cur] -= 1;
            }
        }
        pub fn find_max(&self, x: IntType) -> IntType {
            let mut cur = 0;
            let mut ans = 0;
            for i in (0..self.l).rev() {
                let has_bit = ((x >> i) & 1) as usize;
                if self.node[cur][has_bit ^ 1] > 0 && self.cnt[self.node[cur][has_bit ^ 1]] > 0 {
                    ans += 1 << i;
                    cur = self.node[cur][has_bit ^ 1];
                } else {
                    cur = self.node[cur][has_bit];
                }
            }
            return ans;
        }
    }
}
