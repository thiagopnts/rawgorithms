use std::vec::Vec;

pub struct QuickUnion {
    id: Vec<usize>
}

impl QuickUnion {
    pub fn new(n: usize) -> QuickUnion {
        QuickUnion{ id: Vec::from_fn(n, |i| i) }
    }

    fn root(&self, i: usize) -> usize {
        if i == self.id[i] {
            return i;
        }
        self.root(self.id[i])
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        self.id.remove(i);
        self.id.insert(i, j);
    }
}
