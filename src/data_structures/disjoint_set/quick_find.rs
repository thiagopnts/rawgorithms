
use std::vec::Vec;

pub struct QuickFind {
    id: Vec<usize>
}

impl QuickFind {
    pub fn new(n: usize) -> QuickFind {
        QuickFind{ id: (0 .. n).map(|i| i).collect() }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];
        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }
}

