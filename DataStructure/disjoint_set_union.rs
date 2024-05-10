struct UnionFind {
    d: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            d: (0..=n).collect::<Vec<_>>(),
            sz: vec![1; n + 1],
        }
    }

    pub fn find(&mut self, index: usize) -> usize {
        if self.d[index] == index {
            return index;
        }
        self.d[index] = self.find(self.d[index]);
        self.d[index]
    }

    pub fn len(&mut self, index: usize) -> usize {
        self.sz[self.find(idx)]
    }

    pub fn join(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return false;
        }
        self.sz[a] += self.sz[b];
        self.d[a] = b;
        true
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}
