use std::vec::Vec;

struct QuickUnion {
    id: Vec<uint>
}

impl QuickUnion {
    fn new(n: uint) -> QuickUnion {
        QuickUnion{ id: Vec::from_fn(n, |i| i) }
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
        self.id.remove(i);
        self.id.insert(i, j);
    }
}

fn main() {
    let mut q = QuickUnion::new(3);
    assert_eq!(q.id[0], vec!(0, 1, 2)[0]);
    q.union(0, 1);
    assert_eq!(q.connected(0, 1), true);
    assert_eq!(q.connected(1, 2), false);
}
