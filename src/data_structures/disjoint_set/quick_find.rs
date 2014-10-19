
use std::vec::Vec;

pub struct QuickFind {
    id: Vec<int>
}

impl QuickFind {
    pub fn new(n: uint) -> QuickFind {
        QuickFind{ id: Vec::from_fn(n, |i| i as int) }
    }

    pub fn connected(&self, p: uint, q: uint) -> bool {
        self.id[p] == self.id[q]
    }

    pub fn union(&mut self, p: uint, q: uint) {
        let pid = self.id[p];
        let qid = self.id[q];
        for i in range(0, self.id.len()) {
            if self.id[i] == pid {
                *self.id.get_mut(i) = qid;
            }
        }
    }
}

