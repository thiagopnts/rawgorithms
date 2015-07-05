extern crate rand;

use std::vec::Vec;

use self::rand::random;

// linear time shuffling algorithm
pub fn knuth_shuffle(n: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0 .. n).map(|i| i + 1).collect();
    for i in 0..n {
        let r = random::<usize>() % (i + 1);
        let value: usize = vec[i];
        vec[i] = vec[r];
        vec[r] = value;
    }
    vec
}

pub fn shuffle<T>(v: &mut Vec<T>) where T: Clone, T: PartialOrd {
    for i in 0usize..v.len() {
        let r = random::<usize>() % (i + 1);
        exch(v, i, r);
    }
}

pub fn exch<T: Clone + PartialOrd>(a: &mut Vec<T>, i: usize, j: usize) {
    let from = a[i].clone();
    let to = a[j].clone();
    a[i] = to;
    a[j] = from;
}

pub fn is_sorted<T: PartialOrd>(a: &Vec<T>) -> bool {
    for i in 1..a.len() {
        if a[i - 1] > a[i] { return false; }
    }
    true
}


