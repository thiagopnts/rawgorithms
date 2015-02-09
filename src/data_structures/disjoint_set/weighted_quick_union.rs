#![allow(dead_code)]
#![allow(unused_variable)]

use std::vec::Vec;

pub struct WeightedQuickUnion {
    id: Vec<usize>,
    size: Vec<usize>
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> WeightedQuickUnion {
        WeightedQuickUnion { id: Vec::from_fn(n, |i| i), size: Vec::from_fn(n, |i| 1) }
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
        let current_i = self.size[i];
        let current_j = self.size[j];
        if i == j { return; }
        if self.size[i] < self.size[j] {
            // FIXME
            self.id.remove(i);
            self.id.insert(i, j);
            self.size.remove(j);
            self.size.insert(j, current_j + current_i);
        } else {
            self.id.remove(j);
            self.id.insert(j, i);
            self.size.remove(i);
            self.size.insert(i, current_j + current_i);
        }
    }
}
