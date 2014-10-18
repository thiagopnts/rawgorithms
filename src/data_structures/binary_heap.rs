use std::vec::Vec;
use utils::exch;

pub struct BinaryHeap<T: PartialOrd + Clone> {
    pq: Vec<T>,
    n: uint
}

impl<T: PartialOrd + Clone> BinaryHeap<T> {
    pub fn new(capacity: uint) -> BinaryHeap<T> {
        BinaryHeap { pq: Vec::new(), n: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn insert(&mut self, value: T) {
        if self.is_empty() {
            self.pq.push(value.clone());
        }
        self.pq.push(value);
        self.n += 1;
        let n = self.n;
        self.swim(n);
    }

    pub fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let max = self.pq.remove(1);
        self.n -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, n: uint) {
        let mut k = n;
        while k > 1 && self.pq[k / 2] < self.pq[k] {
            exch(&mut self.pq, k, k / 2);
            k = k / 2;
        }
    }

    fn sink(&mut self, k: uint) {
        let mut i = k;
        while (2 * i) <= self.n {
            let mut j = 2 * i;
            if j < self.n && self.pq[j] < self.pq[j + 1] {
                j += 1
            }
            if !(self.pq[i] < self.pq[j]) {
                break;
            }
            exch(&mut self.pq, i, j);
            i = j
        }
    }
}

#[test]
fn test_binary_heap() {

}
