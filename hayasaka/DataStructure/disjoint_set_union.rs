mod uf {
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        d: Vec<usize>,
        sz: Vec<usize>,
        cc: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                d: (0..=n).collect::<Vec<_>>(),
                sz: vec![1; n + 1],
                cc: n,
            }
        }

        pub fn find(&mut self, index: usize) -> usize {
            if self.d[index] == index {
                return index;
            }
            self.d[index] = self.find(self.d[index]);
            self.d[index]
        }

        pub fn is_root(&self, index: usize) -> bool {
            self.d[index] == index
        }

        pub fn len(&mut self, index: usize) -> usize {
            let nxt = self.find(index);
            self.sz[nxt]
        }

        pub fn num_cc(&self) -> usize {
            self.cc
        }

        pub fn join(&mut self, a: usize, b: usize) -> bool {
            let a = self.find(a);
            let b = self.find(b);
            if a == b {
                return false;
            }
            self.sz[b] += self.sz[a];
            self.cc -= 1;
            self.d[a] = b;
            true
        }

        pub fn is_same(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }
    }
}
