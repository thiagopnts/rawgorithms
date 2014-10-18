
use std::vec::Vec;

pub struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq: Vec<T>
}

// This is a simple priority queue implementation with
// O(1) insertion and O(n) max/min deletion. For a O(log n) insertion/max/min deletion
// check Binary Heap data structure.
impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    pub fn len(&self) -> uint {
        self.pq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    pub fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    pub fn max(&mut self) -> Option<&T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq.get(max))
    }

    pub fn min(&mut self) -> Option<&T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq.get(min))
    }

    pub fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        self.pq.remove(max)
    }

    pub fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        self.pq.remove(min)
    }

    fn max_index(&self) -> uint {
        let mut max = 0u;
        for i in range(1, self.pq.len() - 1) {
            if self.pq[max] < self.pq[i + 1] {
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> uint {
        let mut min = 0u;
        for i in range(0, self.pq.len() - 1) {
            if self.pq[i] < self.pq[i + 1] {
                min = i;
            }
        }
        min
    }
}
