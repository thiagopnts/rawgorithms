
use std::vec::Vec;

pub struct QuickFind {
    id: Vec<int>
}

impl QuickFind {
    fn new(n: uint) -> QuickFind {
        QuickFind{ id: Vec::from_fn(n, |i| i as int) }
    }

    fn connected(&self, p: uint, q: uint) -> bool {
        self.id[p] == self.id[q]
    }

    fn union(&mut self, p: uint, q: uint) {
        let pid = self.id[p];
        let qid = self.id[q];
        for i in range(0, self.id.len()) {
            if self.id[i] == pid {
                *self.id.get_mut(i) = qid;
            }
        }
    }
}

fn main() {
    let mut q = QuickFind::new(10);
    q.union(5, 6);
    q.union(0, 9);
    q.union(9, 3);
    q.union(2, 1);
    q.union(7, 0);
    q.union(4, 8);
    println!("{}", q.connected(6, 5));
    println!("{}", q.connected(0, 1));
}

