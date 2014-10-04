#![allow(dead_code)]
#![allow(unused_variable)]

use std::vec::Vec;

struct WeightedQuickUnion {
    id: Vec<uint>,
    size: Vec<uint>
}

impl WeightedQuickUnion {
    fn new(n: uint) -> WeightedQuickUnion {
        WeightedQuickUnion { id: Vec::from_fn(n, |i| i), size: Vec::from_fn(n, |i| 1) }
    }

    fn root(&self, i: uint) -> uint {
        if i == self.id[i] {
            return i;
        } 
        self.root(self.id[i])
    }

    fn connected(&self, p: uint, q: uint) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: uint, q: uint) {
        let i = self.root(p);
        let j = self.root(q);
        let current_i = self.size[i];
        let current_j = self.size[j];
        if i == j { return; }
        if self.size[i] < self.size[j] {
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

fn main() {
    let mut q = WeightedQuickUnion::new(10);
    q.union(4, 7);
    q.union(3, 1);
    q.union(1, 2);
    q.union(6, 5);
    q.union(9, 2);
    q.union(8, 0);
    q.union(6, 0);
    q.union(4, 3);
    q.union(9, 8);

    println!("{}", q.id);
}
