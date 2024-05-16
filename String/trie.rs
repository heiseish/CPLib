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
