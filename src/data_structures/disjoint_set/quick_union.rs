use std::vec::Vec;

pub struct QuickUnion {
    id: Vec<uint>
}

impl QuickUnion {
    pub fn new(n: uint) -> QuickUnion {
        QuickUnion{ id: Vec::from_fn(n, |i| i) }
    }

    fn root(&self, i: uint) -> uint {
        if i == self.id[i] {
            return i;
        } 
        self.root(self.id[i])
    }

    pub fn connected(&self, p: uint, q: uint) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: uint, q: uint) {
        let i = self.root(p);
        let j = self.root(q);
        self.id.remove(i);
        self.id.insert(i, j);
    }
}
