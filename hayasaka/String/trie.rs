mod trie {
    const ALPHABET_SIZE: usize = 30;

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
}

mod trie {
    type IntType = i64;

    const ROOT: usize = 0;

    #[derive(Default, Clone)]
    pub struct Node {
        child: [usize; 2],
        f: IntType,
    }

    #[derive(Default, Clone)]
    pub struct BinaryTrie {
        l: usize,
        c: usize,
        t: Vec<Node>,
    }

    impl BinaryTrie {
        pub fn new(n: usize) -> Self {
            let num_bits = std::mem::size_of::<IntType>() * 8;
            Self {
                l: num_bits,
                c: 1,
                t: vec![Node::default(); (num_bits + 2) * n + 3],
            }
        }

        pub fn add(&mut self, val: IntType, idx: IntType) {
            let mut u = ROOT;
            for i in (0..self.l).rev() {
                let x = ((val >> i) & 1) as usize;

                if self.t[u].child[x] == 0 {
                    self.t[u].child[x] = self.c;
                    self.c += 1;
                }

                u = self.t[u].child[x];
                self.t[u].f = self.t[u].f.max(idx);
            }
        }

        pub fn remove(&mut self, val: IntType) {
            let mut u = 0;
            for i in (0..self.l).rev() {
                let x = ((val >> i) & 1) as usize;

                if self.t[u].child[x] == 0 {
                    self.t[u].child[x] = self.c;
                    self.c += 1;
                }
                u = self.t[u].child[x];
                // self.cnt[u] -= 1;
            }
        }

        pub fn get(&self, val: IntType, req: IntType) -> IntType {
            let mut u = ROOT;
            let mut ans = 0;
            for i in (0..self.l).rev() {
                let x = ((val >> i) & 1) as usize;
                let y = ((req >> i) & 1) as usize;

                if y > 0 {
                    ans = ans.max(self.t[self.t[u].child[x]].f);
                }
                u = self.t[u].child[x ^ y];
                if u == ROOT {
                    return ans;
                }
            }
            ans.max(self.t[u].f)
        }
    }
}
use trie::*;
